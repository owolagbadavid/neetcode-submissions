impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let total: i32 = nums.iter().sum();

        for i in 0..nums.len() {
            sum += nums[i];
            if sum - nums[i] == total - sum {
                return i as i32;
            }
        }

        -1
    }
}
