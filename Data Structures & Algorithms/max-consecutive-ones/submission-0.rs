impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut res = 0;

        let mut l = 0;
        for r in 0..nums.len() {
            if nums[r] == 0 {
                l = r+1;
            }
            res = res.max(r - l + 1);
        }

        res as i32
    }
}
