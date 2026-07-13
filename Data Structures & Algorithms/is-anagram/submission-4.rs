impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut counter = HashMap::new();
        for c in s.chars() {
            let count = counter.entry(c).or_insert(0);
            *count += 1
        }

        for c in t.chars() {
            let count = counter.entry(c).or_insert(0);
            *count -= 1;
            if *count < 0 {
                return false;
            }
        }

        true
    }
}
