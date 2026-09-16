impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut prefix = vec![0; nums.len()];
        let mut sum = 0;

        for i in 0..nums.len() {
            sum += nums[i];
            prefix[i] = sum;
        }

        for i in 0..nums.len() {
            if prefix[i] - nums[i] == prefix[nums.len()-1] - prefix[i] {
                return i as i32;
            }
        }

        -1
    }
}
