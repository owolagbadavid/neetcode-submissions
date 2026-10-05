impl Solution {
    pub fn max_score(s: String) -> i32 {
        let b = s.as_bytes();
        let mut left = vec![0; s.len()];
        let mut right = vec![0; s.len()];
        let mut res = 0;

        for i in 0..s.len() {
            left[i] = if i > 0 {left[i-1]} else {0};
            if b[i] == b'0' {
                left[i] += 1;
            }
        }

        for i in (0..s.len()).rev() {
            right[i] = if i < s.len()-1 {right[i+1]} else {0};
            if b[i] == b'1' {
                right[i] += 1;
            }
        }

        for i in 0..s.len()-1 {
            res = res.max(left[i]+right[i+1]);
        }

        res
    }
}