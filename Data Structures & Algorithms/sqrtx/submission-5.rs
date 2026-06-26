impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut l: i64 = 0;
        let mut r: i64 = x as i64;
        let target = r;
        while l <= r {
            let m = (l+r) / 2;
            let sqr = m * m;
            if sqr == target {
                return m as i32;
            } else if sqr < target {
                l = m+1;
            } else {
                r = m-1;
            }
        }

        return r as i32;
    }
}
