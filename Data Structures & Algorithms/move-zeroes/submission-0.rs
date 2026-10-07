impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut l = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                (nums[i], nums[l]) = (nums[l], nums[i]);
                l += 1;
            }
        }
    }
}
