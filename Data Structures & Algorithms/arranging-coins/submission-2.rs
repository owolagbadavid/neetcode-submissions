impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let (mut l, mut r) = (1i64, n as i64);
        let mut res = 0i64;

        while l <= r {
            let m = (l + r) / 2;
            let coins = (m as f64 / 2.) * (m as f64 + 1.);
            if coins > n as f64 {
                r = m - 1;
            } else {
                l = m + 1;
                res = res.max(m);
            }
        }

        res as i32
    }
}
