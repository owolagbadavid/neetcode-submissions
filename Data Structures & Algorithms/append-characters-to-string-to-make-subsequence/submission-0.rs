impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let (mut i, mut j) = (0, 0);
        let s = s.as_bytes();
        let t = t.as_bytes();

        while i < s.len() && j < t.len() {
            if s[i] == t[j] {
                j += 1;
            }
            i += 1;
        }

        (t.len() - j) as i32
    }
}
