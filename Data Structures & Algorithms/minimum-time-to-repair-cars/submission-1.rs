impl Solution {
    pub fn repair_cars(mut ranks: Vec<i32>, cars: i32) -> i64 {
        ranks.sort();
        let cars = cars as i64;
        let mut l = 1i64;
        let mut r = *ranks.last().unwrap() as i64 * cars * cars;

        let mut res = r;

        let is_valid = |m| {
            let mut cars = cars;

            for &rank in &ranks {
                let mut res = 0;
                let mut l = 0;
                let mut r = cars;
                while l <= r {
                    let mid = (l + r) / 2;
                    if mid * mid * rank as i64 <= m {
                        res = res.max(mid);
                        l = mid + 1;
                    } else {
                        r = mid - 1;
                    }
                }
                cars -= res;
            }

            cars <= 0
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
