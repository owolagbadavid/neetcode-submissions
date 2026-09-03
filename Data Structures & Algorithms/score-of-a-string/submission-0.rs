impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut res = 0;
        let s = s.as_bytes();

        // s.len() >= 2
        for i in 1..s.len() {
            res += (s[i] as i32 - s[i-1] as i32).abs()
        }

        res
    }
}
