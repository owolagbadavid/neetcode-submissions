impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool { 
        if pattern.len() != s.split(' ').count() {
            return false;
        }

        let mut str_map: HashMap<&str, u8> = HashMap::new();
        let mut byte_map: HashMap<u8, &str> = HashMap::new();

        let pattern = pattern.as_bytes();
        let mut i = 0;
        for sub in s.split(' ') {
            let str_item = *str_map.entry(sub).or_insert(pattern[i]);
            let byte_item = *byte_map.entry(pattern[i]).or_insert(sub);
            if str_item != pattern[i] || byte_item != sub {
                return false;
            }
            i += 1;
        }

        true
    }
}
