impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let total =  candies.iter().map(|n| *n as i64).sum::<i64>();
        if total < k {
            return 0;
        }

        let mut l = 1;
        let mut r = total / k;
        let mut res = 1;

        while l <= r {
            let m = (r + l) / 2;
            let mut rem = k;
            for &n in &candies {
                rem -= n as i64 / m;
            }

            if rem <= 0 {
                res = res.max(m);
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        res as i32
    }
}
