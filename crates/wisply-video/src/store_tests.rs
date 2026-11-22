use serde_json::json;

use super::*;

const PLAIN: &str = "ba58db519ce2190b37a01f0b2b549835fdb51608c626f724f59d5925d492e4be";
const ODD: &str = "b5152af0736239be75a25e62a8f67c674f1c194903748009698e017a74c8cdc3";

fn key(section: &str, mode: &str, focus: &str, model: &str, profile: &str) -> AnalysisKey {
    AnalysisKey {
        video_id: "dQw4w9WgXcQ".to_string(),
        section: section.to_string(),
        mode: mode.to_string(),
        focus: normalize_focus(focus),
        prompt_version: "3".to_string(),
        model: model.to_string(),
        profile: profile.to_string(),
    }
}

fn plain() -> AnalysisKey {
    key(WHOLE_VIDEO, "steps", "  How to   BEVEL ", "gemini-3.8-flash", "medium")
}

fn folder(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("wisply-video-store-{name}-{}", std::process::id()))
}

#[test]
fn the_digest_matches_the_one_iris_wrote() {
    assert_eq!(plain().digest(), PLAIN);
    assert_eq!(key("60s-120s", "full", "Straße \"A\"\\B\tÜber", "gemini-3.5-flash-lite", "low").digest(), ODD);
    assert_eq!(normalize_focus("Straße \"A\"\\B\tÜber"), "strasse \"a\"\\b über");
    assert_eq!(normalize_focus("   "), "");
}

#[test]
fn a_new_cache_gets_every_table() {
    let store = Store::in_memory();
    assert_eq!(store.version().unwrap(), 4);
    let sql = "SELECT COUNT(*) FROM sqlite_master WHERE name IN ('video', 'analysis', 'quota_event', 'quota_block', \
               'quota_learned', 'transcript', 'transcript_part', 'transcript_search', 'spend', 'answer')";
    let tables: i64 = store.read().query_row(sql, [], |row| row.get(0)).unwrap();
    assert_eq!(tables, 10);
}

#[test]
fn a_file_is_reopened_without_running_the_schema_again() {
    let path = folder("reopen").join("deep").join("video.db");
    let first = Store::open(&path, 1.0).unwrap();
    first.put_analysis(&plain(), &json!({"steps": ["one"]}), &json!({"tokens": 7}), 100.0).unwrap();
    drop(first);
    let again = Store::open(&path, 1.0).unwrap();
    assert_eq!(again.version().unwrap(), 4);
    assert_eq!(again.count_answers().unwrap(), 1);
    drop(again);
    std::fs::remove_dir_all(folder("reopen")).unwrap();
}

#[test]
fn a_cache_from_a_newer_wisply_is_refused() {
    let path = folder("newer").join("video.db");
    let store = Store::open(&path, 1.0).unwrap();
    store.read().pragma_update(None, "user_version", 9).unwrap();
    drop(store);
    let fail = Store::open(&path, 1.0).err().unwrap();
    assert_eq!(fail.kind, Kind::Storage);
    assert!(fail.message.contains("newer wisply") && fail.hint.contains("video.db"), "{}", fail.text());
    std::fs::remove_dir_all(folder("newer")).unwrap();
}

#[test]
fn a_file_that_is_no_database_is_named_as_damaged() {
    let path = folder("damaged").join("video.db");
    std::fs::create_dir_all(folder("damaged")).unwrap();
    std::fs::write(&path, "this is not a database, it is a text file with enough bytes to look like a page").unwrap();
    let fail = Store::open(&path, 1.0).err().unwrap();
    assert_eq!(fail.kind, Kind::Storage);
    assert!(fail.hint.contains("delete video.db"), "{}", fail.text());
    std::fs::remove_dir_all(folder("damaged")).unwrap();
}

#[test]
fn an_analysis_comes_back_as_it_was_stored() {
    let store = Store::in_memory();
    assert_eq!(store.analysis(&plain()).unwrap(), None);
    let payload = json!({"title": "Über \"Bevel\"", "steps": [{"at": 12.5, "text": "press Ctrl+B"}]});
    store.put_analysis(&plain(), &payload, &json!({"tokens": 4100}), 100.0).unwrap();
    let cached = store.analysis(&plain()).unwrap().unwrap();
    assert_eq!((cached.payload, cached.usage, cached.created_at), (payload, json!({"tokens": 4100}), 100.0));
    store.put_analysis(&plain(), &json!({"steps": []}), &json!({}), 200.0).unwrap();
    assert_eq!(store.analysis(&plain()).unwrap().unwrap().created_at, 200.0);
    assert_eq!(store.count_answers().unwrap(), 1);
}

#[test]
fn another_focus_model_or_section_is_another_entry() {
    let store = Store::in_memory();
    store.put_analysis(&plain(), &json!(1), &json!({}), 1.0).unwrap();
    let others = [
        key(WHOLE_VIDEO, "steps", "how to bevel", "gemini-3.6-flash", "medium"),
        key(WHOLE_VIDEO, "steps", "how to inset", "gemini-3.8-flash", "medium"),
        key("60s-120s", "steps", "how to bevel", "gemini-3.8-flash", "medium"),
        key(WHOLE_VIDEO, "full", "how to bevel", "gemini-3.8-flash", "medium"),
        key(WHOLE_VIDEO, "steps", "how to bevel", "gemini-3.8-flash", "low"),
    ];
    for other in &others {
        assert_eq!(store.analysis(other).unwrap(), None, "{other:?}");
    }
    assert!(
        store.analysis(&key(WHOLE_VIDEO, "steps", "HOW TO bevel", "gemini-3.8-flash", "medium")).unwrap().is_some()
    );
}

#[test]
fn video_info_is_only_given_while_it_is_fresh() {
    let store = Store::in_memory();
    assert_eq!(store.video_info("dQw4w9WgXcQ", 3600.0, 1000.0).unwrap(), None);
    store.put_video_info("dQw4w9WgXcQ", &json!({"title": "Donut", "duration": 600}), 1000.0).unwrap();
    assert_eq!(
        store.video_info("dQw4w9WgXcQ", 3600.0, 4600.0).unwrap(),
        Some(json!({"title": "Donut", "duration": 600}))
    );
    assert_eq!(store.video_info("dQw4w9WgXcQ", 3600.0, 4601.0).unwrap(), None);
    store.put_video_info("dQw4w9WgXcQ", &json!({"title": "Donut 2"}), 5000.0).unwrap();
    assert_eq!(store.video_info("dQw4w9WgXcQ", 3600.0, 5001.0).unwrap(), Some(json!({"title": "Donut 2"})));
}
