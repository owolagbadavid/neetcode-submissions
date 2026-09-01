impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut i = 0;
        let mut j = 0;

        let (s, t) = (s.as_bytes(), t.as_bytes());

        if s.len() == 0 {
            return true;
        }

        while j < t.len() {
            if s[i] == t[j] {
                i += 1;
                if i == s.len() { return true; }
            }
            j += 1;
        }

        false
    }
}
