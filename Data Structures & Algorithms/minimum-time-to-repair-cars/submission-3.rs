impl Solution {
    pub fn repair_cars(mut ranks: Vec<i32>, cars: i32) -> i64 {
        let cars = cars as i64;
        let mut l = 1i64;
        let mut r = *ranks.last().unwrap() as i64 * cars * cars;

        let mut res = r;

        let is_valid = |m: i64| {
            let mut left = cars;
            for &rank in &ranks {
                left -= (m / rank as i64).isqrt();
                if left <= 0 {
                    return true;
                }
            }
            false
        };

        while l <= r {
            let m = (r + l) / 2;

            if is_valid(m) {
                res = res.min(m);
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        res
    }
}
