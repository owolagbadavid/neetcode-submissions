impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut count = HashMap::new();

        for c in magazine.chars() {
            *count.entry(c).or_insert(0) += 1;
        }

        for c in ransom_note.chars() {
            if let Some(entry) = count.get_mut(&c) {
                *entry -= 1;
                if *entry < 0 {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}
