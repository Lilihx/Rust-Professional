use std::{collections::HashMap, hash::Hash};

use serde_json::{json, Value};
pub fn count_provinces() -> String {
    let file = std::fs::read_to_string("./district.json").unwrap();
    let data: Value = serde_json::from_str(&file).unwrap();
    let mut hs: HashMap<&str, HashMap<&str, Vec<&str>>> = HashMap::new();

    String::new()
}
