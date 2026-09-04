impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut res = 0;
        let s = s.trim().as_bytes();

        for i in (0..s.len()).rev() {
            if s[i] == b' ' { break; } else { res += 1; }
        }

        res
    }
}
