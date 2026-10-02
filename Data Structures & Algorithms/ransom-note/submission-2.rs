impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut count = HashMap::new();
        let mut default = 0;

        for c in magazine.chars() {
            *count.entry(c).or_insert(0) += 1;
        }

        for c in ransom_note.chars() {
            let entry = count.get_mut(&c).unwrap_or(&mut default);
            *entry -= 1;
            if *entry < 0 {
                return false;
            }
        }

        true
    }
}
