impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut l = 0;
        let mut r = x;
        let mut res = 0;
        while l <= r {
            let m = ((l+r) / 2) as i64;
            let sqr: i64 = m * m;
            if sqr == x as i64 {
                return m as i32;
            } else if sqr < x as i64 {
                res = m as i32;
                l = (m+1) as i32;
            } else {
                r = (m-1) as i32;
            }
        }

        return res;
    }
}
