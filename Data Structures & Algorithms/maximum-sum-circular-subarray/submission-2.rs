impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut min = nums[0];
        let mut cur_min = 0;
        let mut cur_max = 0;
        let mut sum = 0;
        let n = nums.len();
        for i in 0..n {
            if cur_max < 0 {
                cur_max = 0;
            }
            if cur_min > 0 {
                cur_min = 0;
            }
            cur_min += nums[i];
            cur_max += nums[i];
            sum += nums[i];
            max = max.max(cur_max);
            min = min.min(cur_min);
        }

        if max < 0 {max} else {max.max(sum - min)}
    }
}
