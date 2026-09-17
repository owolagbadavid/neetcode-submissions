use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut map = HashMap::new();

        for s in &arr {
            *map.entry(s).or_insert(0) += 1;
        }

        let mut i = 1;
        for s in &arr {
            if map[&s] != 1 {
                continue;
            }

            if i == k {
                return String::from(s);
            }
            i += 1;
        }

        String::new()
    }
}
