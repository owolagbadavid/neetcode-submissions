impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = nums.len();

        while l < r {
            let m = (l + r) / 2;
            if nums[m] >= target {
                r = m;
            } else if nums[m] < target {
                l = m + 1;
            }
        }

        l as i32
    }
}
