impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut min = nums[0];
        let mut curMin = 0;
        let mut curMax = 0;
        let mut sum = 0;
        let n = nums.len();
        for i in 0..n {
            if curMax < 0 {
                curMax = 0;
            }
            if curMin > 0 {
                curMin = 0;
            }
            curMin += nums[i];
            curMax += nums[i];
            sum += nums[i];
            max = max.max(curMax);
            min = min.min(curMin);
        }

        if max < 0 {max} else {max.max(sum - min)}
    }
}
