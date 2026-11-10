use serde_json::{Map, Value};

const PARTS: [&str; 3] = ["pages", "fixes", "stats"];
const ENTRY_FIELDS: usize = 3;

fn object(v: &mut Value) -> bool {
    if v.is_object() {
        return false;
    }
    *v = Value::Object(Map::new());
    true
}

fn keep(map: &mut Map<String, Value>, good: impl Fn(&Value) -> bool) -> bool {
    let before = map.len();
    map.retain(|_, v| good(v));
    map.len() != before
}

pub fn mend(data: &mut Value) -> bool {
    let mut changed = object(data);
    for part in PARTS {
        changed |= object(&mut data[part]);
    }
    if let Some(pages) = data["pages"].as_object_mut() {
        changed |= keep(pages, Value::is_object);
        for page in pages.values_mut() {
            changed |= object(&mut page["els"]);
            if let Some(els) = page["els"].as_object_mut() {
                changed |= keep(els, |e| e.as_array().is_some_and(|a| a.len() >= ENTRY_FIELDS));
            }
        }
    }
    if let Some(fixes) = data["fixes"].as_object_mut() {
        changed |= keep(fixes, Value::is_object);
    }
    changed
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn a_whole_file_stays_as_it_is() {
        let mut d = json!({
            "v": 1,
            "pages": {"shop.example": {"uses": 3, "t": 5.0, "els": {"buy": [1.0, 5.0, "button", "Buy"]}}},
            "fixes": {"shop.example": {"click|by": ["Buy", 2, 5.0]}},
            "stats": {"pruned": 4}
        });
        let before = d.clone();
        assert!(!mend(&mut d));
        assert_eq!(d, before);
    }

    #[test]
    fn broken_parts_are_dropped_and_the_rest_is_kept() {
        let mut d = json!({
            "v": 1,
            "pages": {
                "good.example": {"uses": 2, "t": 1.0, "els": {"ok": [1.0, 1.0, "link"], "short": [1.0], "text": "x"}},
                "noels.example": {"uses": 1},
                "number.example": 7
            },
            "fixes": {"good.example": {"click|x": ["X", 1, 1.0]}, "bad.example": [1, 2]},
            "stats": "many"
        });
        assert!(mend(&mut d));
        let want = json!({
            "v": 1,
            "pages": {
                "good.example": {"uses": 2, "t": 1.0, "els": {"ok": [1.0, 1.0, "link"]}},
                "noels.example": {"uses": 1, "els": {}}
            },
            "fixes": {"good.example": {"click|x": ["X", 1, 1.0]}},
            "stats": {}
        });
        assert_eq!(d, want);
        assert!(!mend(&mut d));
    }

    #[test]
    fn missing_parts_are_added() {
        for mut d in [json!({"v": 1}), json!({"v": 1, "pages": [], "fixes": null}), json!([1, 2])] {
            assert!(mend(&mut d));
            assert!(d["pages"].is_object() && d["fixes"].is_object() && d["stats"].is_object());
        }
    }

    #[test]
    fn a_mended_file_can_be_learned_into() {
        let mut d = json!({
            "v": 1,
            "pages": {"shop.example": {"uses": 1, "els": {"buy": [1.0]}}, "x.example": 3},
            "fixes": 5
        });
        mend(&mut d);
        super::super::learn(&mut d, "shop.example/cart", "Buy", &json!("button"), 10.0);
        super::super::trim(&mut d);
        assert_eq!(d["pages"]["shop.example"]["uses"], 2);
        assert_eq!(d["pages"]["shop.example"]["els"]["buy"], json!([1.0, 10.0, "button", "Buy"]));
        assert_eq!(d["pages"]["shop.example/cart"]["uses"], 1);
    }
}
