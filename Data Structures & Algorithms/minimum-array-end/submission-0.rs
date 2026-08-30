impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let mut i = 1;
        let mut cur = x as i64;

        while i < n {
            cur = (cur as i64 + 1) | x as i64;
            i += 1;
        }

        cur
    }
}
