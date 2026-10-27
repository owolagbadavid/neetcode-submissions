impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut l = 0;
        let n = nums.len();
        let mut res = 0;
        let mut sum = 0;

        for r in 0..n {
            sum += nums[r];

            while sum > goal {
                sum -= nums[l];
                l += 1;
            }

            if sum <= goal {
                res += r-l+1;
            }
        }
        sum = 0;
        l = 0;
        for r in 0..n {
            sum += nums[r];

            while l <= r && sum > goal-1 {
                sum -= nums[l];
                l += 1;
            }

            if sum <= goal-1 {
                res -= r-l+1;
            }
        }

        res as i32
    }
}
