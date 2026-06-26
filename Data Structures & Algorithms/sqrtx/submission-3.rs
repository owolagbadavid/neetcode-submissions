impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut l: i64 = 0;
        let mut r: i64 = x as i64;
        let mut res = l;
        while l <= r {
            let m = (l+r) / 2;
            let sqr = m * m;
            if sqr == x as i64 {
                return m as i32;
            } else if sqr < x as i64 {
                res = m;
                l = m+1;
            } else {
                r = m-1;
            }
        }

        return res as i32;
    }
}
