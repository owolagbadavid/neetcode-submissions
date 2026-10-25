impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let mut res = 0;
        let mut l = 0;

        let mut sum = 0;

        let s = s.as_bytes();
        let t = t.as_bytes();

        for r in 0..s.len() {
            sum += (s[r] as i32 - t[r] as i32).abs();

            while sum > max_cost {
                sum -= (s[l] as i32 - t[l] as i32).abs();
                l += 1;
            }

            res = res.max(r-l+1);
        }

        res as i32
    }
}
