use std::sync::LazyLock;

use regex::Regex;
use rusqlite::{OptionalExtension, Row, Transaction, params};
use serde_json::{Value, json};

use crate::captions::Segment;
use crate::fail::Fail;
use crate::store::Store;

const PREFIX_CHARS: usize = 4;
const THOUSANDTHS: f64 = 1000.0;
const ANY_OF: &str = " OR ";

static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\w+").expect("word"));

#[derive(Debug, Clone, PartialEq)]
pub struct Stored {
    pub kind: String,
    pub language: String,
    pub note: String,
    pub created_at: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Part {
    pub part: i64,
    pub start: f64,
    pub end: f64,
    pub segments: Vec<Segment>,
}

fn rounded(seconds: f64) -> f64 {
    (seconds * THOUSANDTHS).round() / THOUSANDTHS
}

pub fn segments_to_json(segments: &[Segment]) -> Value {
    let rows = segments.iter().map(|segment| json!([rounded(segment.start), rounded(segment.end), segment.text]));
    Value::Array(rows.collect())
}

pub fn segments_from_json(value: &Value) -> Vec<Segment> {
    let read = |row: &Value| {
        let row = row.as_array()?;
        Some(Segment::new(row.first()?.as_f64()?, row.get(1)?.as_f64()?, row.get(2)?.as_str()?))
    };
    value.as_array().map(|rows| rows.iter().filter_map(read).collect()).unwrap_or_default()
}

pub fn search_expression(query: &str) -> Option<String> {
    let lowered = query.to_lowercase();
    let mut words: Vec<&str> = Vec::new();
    for found in WORD.find_iter(&lowered) {
        if !words.contains(&found.as_str()) {
            words.push(found.as_str());
        }
    }
    let term = |word: &&str| {
        if word.chars().count() >= PREFIX_CHARS { format!("\"{word}\"*") } else { format!("\"{word}\"") }
    };
    let terms: Vec<String> = words.iter().map(term).collect();
    if terms.is_empty() { None } else { Some(terms.join(ANY_OF)) }
}

fn write_part(tx: &Transaction<'_>, key: &str, part: &Part) -> Result<(), Fail> {
    tx.execute("DELETE FROM transcript_search WHERE video_key = ?1 AND part = ?2", params![key, part.part])?;
    let segments = serde_json::to_string(&segments_to_json(&part.segments))?;
    let values = params![key, part.part, rounded(part.start), rounded(part.end), segments];
    tx.execute("INSERT OR REPLACE INTO transcript_part VALUES (?1, ?2, ?3, ?4, ?5)", values)?;
    let sql = "INSERT INTO transcript_search (text, video_key, part, start_seconds, end_seconds)
               VALUES (?1, ?2, ?3, ?4, ?5)";
    let mut insert = tx.prepare(sql)?;
    for segment in &part.segments {
        insert.execute(params![segment.text, key, part.part, rounded(segment.start), rounded(segment.end)])?;
    }
    Ok(())
}

fn segment(row: &Row<'_>) -> rusqlite::Result<Segment> {
    Ok(Segment { start: row.get(0)?, end: row.get(1)?, text: row.get(2)? })
}

impl Store {
    pub fn transcript(&self, key: &str) -> Result<Option<Stored>, Fail> {
        let sql = "SELECT kind, language, note, created_at FROM transcript WHERE video_key = ?1";
        let read = |row: &Row<'_>| {
            Ok(Stored { kind: row.get(0)?, language: row.get(1)?, note: row.get(2)?, created_at: row.get(3)? })
        };
        Ok(self.read().query_row(sql, [key], read).optional()?)
    }

    pub fn start_transcript(&self, key: &str, stored: &Stored, parts: &[Part]) -> Result<(), Fail> {
        let tx = self.write()?;
        tx.execute("DELETE FROM transcript_search WHERE video_key = ?1", [key])?;
        tx.execute("DELETE FROM transcript_part WHERE video_key = ?1", [key])?;
        let values = params![key, stored.kind, stored.language, stored.note, stored.created_at];
        tx.execute("INSERT OR REPLACE INTO transcript VALUES (?1, ?2, ?3, ?4, ?5)", values)?;
        for part in parts {
            write_part(&tx, key, part)?;
        }
        Ok(tx.commit()?)
    }

    pub fn transcript_parts(&self, key: &str) -> Result<Vec<Part>, Fail> {
        let sql = "SELECT part, start_seconds, end_seconds, segments FROM transcript_part
                   WHERE video_key = ?1 ORDER BY part";
        let mut query = self.read().prepare(sql)?;
        let rows = query.query_map([key], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, f64>(1)?, row.get::<_, f64>(2)?, row.get::<_, String>(3)?))
        })?;
        let mut parts = Vec::new();
        for row in rows {
            let (part, start, end, segments) = row?;
            let segments = segments_from_json(&serde_json::from_str(&segments)?);
            parts.push(Part { part, start, end, segments });
        }
        Ok(parts)
    }

    pub fn search_transcript(
        &self,
        key: &str,
        query: &str,
        start: f64,
        end: Option<f64>,
        limit: usize,
    ) -> Result<Vec<Segment>, Fail> {
        let Some(expression) = search_expression(query) else { return Ok(Vec::new()) };
        let sql = "SELECT CAST(start_seconds AS REAL), CAST(end_seconds AS REAL), text FROM transcript_search
                   WHERE transcript_search MATCH ?1 AND video_key = ?2 AND CAST(end_seconds AS REAL) >= ?3
                     AND (?4 IS NULL OR CAST(start_seconds AS REAL) <= ?4)
                   ORDER BY bm25(transcript_search) LIMIT ?5";
        let mut found = self.read().prepare(sql)?;
        let limit = i64::try_from(limit).unwrap_or(i64::MAX);
        let rows = found.query_map(params![expression, key, start, end, limit], segment)?;
        Ok(rows.collect::<rusqlite::Result<Vec<Segment>>>()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &str = "abcdefghijk";
    const OTHER: &str = "zyxwvutsrqp";

    fn stored(kind: &str) -> Stored {
        Stored { kind: kind.to_string(), language: "en".to_string(), note: String::new(), created_at: 5.0 }
    }

    fn part(number: i64, lines: &[(f64, f64, &str)]) -> Part {
        let segments: Vec<Segment> = lines.iter().map(|(start, end, text)| Segment::new(*start, *end, *text)).collect();
        let (start, end) = (lines.first().map_or(0.0, |line| line.0), lines.last().map_or(0.0, |line| line.1));
        Part { part: number, start, end, segments }
    }

    fn filled() -> Store {
        let store = Store::in_memory();
        let lines =
            [(0.0, 10.0, "select the cube"), (10.0, 20.0, "bevel the edges"), (20.0, 30.0, "now bevelling a lot")];
        store.start_transcript(KEY, &stored("captions"), &[part(0, &lines)]).unwrap();
        store.start_transcript(OTHER, &stored("subtitles"), &[part(0, &[(0.0, 5.0, "bevel elsewhere")])]).unwrap();
        store
    }

    fn texts(found: &[Segment]) -> Vec<&str> {
        found.iter().map(|segment| segment.text.as_str()).collect()
    }

    #[test]
    fn lines_are_stored_with_three_decimals() {
        let value = segments_to_json(&[Segment::new(1.23456, 2.0, "x")]);
        assert_eq!(value, json!([[1.235, 2.0, "x"]]));
        assert_eq!(segments_from_json(&value), [Segment::new(1.235, 2.0, "x")]);
        assert_eq!(segments_from_json(&json!([[1, 2, "whole"], [1.0, "x"], "junk", [1.0, 2.0, 3]])).len(), 1);
        assert!(segments_from_json(&json!({"not": "a list"})).is_empty());
    }

    #[test]
    fn a_transcript_comes_back_as_it_was_stored() {
        let store = Store::in_memory();
        assert_eq!(store.transcript(KEY).unwrap(), None);
        assert!(store.transcript_parts(KEY).unwrap().is_empty());
        let parts = [part(0, &[(0.0, 4.5, "one"), (4.5, 9.0, "two")]), part(1, &[(9.0, 12.0, "three")])];
        store.start_transcript(KEY, &stored("subtitles"), &parts).unwrap();
        assert_eq!(store.transcript(KEY).unwrap(), Some(stored("subtitles")));
        assert_eq!(store.transcript_parts(KEY).unwrap(), parts);
    }

    #[test]
    fn a_new_transcript_replaces_the_old_one_completely() {
        let store = filled();
        let note = Stored { note: "the video has no sound".to_string(), ..stored("none") };
        store.start_transcript(KEY, &note, &[]).unwrap();
        assert_eq!(store.transcript(KEY).unwrap(), Some(note));
        assert!(store.transcript_parts(KEY).unwrap().is_empty());
        assert!(store.search_transcript(KEY, "bevel", 0.0, None, 8).unwrap().is_empty());
        assert_eq!(store.search_transcript(OTHER, "bevel", 0.0, None, 8).unwrap().len(), 1);
    }

    #[test]
    fn words_are_searched_inside_one_video() {
        let store = filled();
        let mut found = store.search_transcript(KEY, "Bevel!", 0.0, None, 8).unwrap();
        found.sort_by(|first, second| first.start.total_cmp(&second.start));
        assert_eq!(texts(&found), ["bevel the edges", "now bevelling a lot"]);
        assert_eq!((found[0].start, found[0].end), (10.0, 20.0));
        assert_eq!(texts(&store.search_transcript(KEY, "now", 0.0, None, 8).unwrap()), ["now bevelling a lot"]);
        assert!(store.search_transcript(KEY, "no", 0.0, None, 8).unwrap().is_empty());
        assert_eq!(store.search_transcript(KEY, "the", 0.0, None, 1).unwrap().len(), 1);
        assert!(store.search_transcript(KEY, " ?! ", 0.0, None, 8).unwrap().is_empty());
    }

    #[test]
    fn a_search_can_stay_inside_a_time_window() {
        let store = filled();
        assert_eq!(texts(&store.search_transcript(KEY, "bevel", 21.0, None, 8).unwrap()), ["now bevelling a lot"]);
        assert_eq!(texts(&store.search_transcript(KEY, "bevel", 0.0, Some(15.0), 8).unwrap()), ["bevel the edges"]);
        assert!(store.search_transcript(KEY, "bevel", 0.0, Some(5.0), 8).unwrap().is_empty());
    }

    #[test]
    fn a_search_reads_every_word_once() {
        assert_eq!(search_expression("Bevel, bevel the cube!").unwrap(), "\"bevel\"* OR \"the\" OR \"cube\"*");
        assert_eq!(search_expression("  !!"), None);
    }
}
