use std::collections::{HashMap, HashSet};

pub fn new_count_distinct(input_str: &str) -> usize {
    let mut hs: HashMap<&str, ()> = HashMap::new();
    for item in input_str.split(",") {
        hs.insert(item, ());
    }
    hs.len()
}
