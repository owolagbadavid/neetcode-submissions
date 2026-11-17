impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let mut l = 1;
        let mut r = *nums.iter().max().unwrap();

        let is_valid = |c| {
            let mut i = 0;
            let mut count = 0;
            while i < nums.len() {
                if nums[i] <= c {
                    count += 1;
                    i += 2;
                } else {
                    i += 1;
                }

                if count == k {
                    break;
                }
            }

            count == k
        };

        let mut res = r;

        while l <= r {
            let m = (l + r) / 2;

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
