impl Solution {
    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
        nums.sort();

        let mut l = 0;
        let mut r = nums[nums.len() - 1] - nums[0];

        let mut res = r;

        let is_valid = |m| {
            let mut have = 0;
            let mut i = 0;
            while i < nums.len() {
                if i + 1 < nums.len() && nums[i+1] - nums[i] <= m {
                    have += 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }

            have >= p
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
