impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let at_most = |k| {
            if k < 0 { return 0; }
            let (mut l, mut res, mut sum) = (0, 0, 0);
            let n = nums.len();

            for r in 0..n {
                sum += nums[r];

                while sum > k {
                    sum -= nums[l];
                    l += 1;
                }

                if sum <= k {
                    res += r-l+1;
                }
            }

            res as i32
        };

        at_most(goal) - at_most(goal-1)
    }
}
