impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        let mut last = -101;

        for r in 0..nums.len() {
            if nums[r] != last {
                nums[i] = nums[r];
                i += 1;
                last = nums[r];
            }
        }

        i as i32
    }
}
