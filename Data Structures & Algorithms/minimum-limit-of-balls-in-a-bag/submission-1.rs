impl Solution {
    pub fn minimum_size(nums: Vec<i32>, maxOperations: i32) -> i32 {
        let mut l = 1;
        let mut r = *nums.iter().max().unwrap();

        let is_valid = |m| {
            let mut left = maxOperations;
            for &num in &nums {
                left -= (num - 1) / m;
            }

            left >= 0
        };

        let mut res = r;

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
