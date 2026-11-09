impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        let mut nums = nums.into_iter().collect::<HashSet<i32>>().into_iter().collect::<Vec<i32>>();
        nums.sort();

        let mut l = 0;
        for r in 0..nums.len() {
            while l <= r && nums[r] as usize >= (nums[l] as usize) + n {
                l += 1;
            }
            res = res.max(r-l+1);
        }

        (n - res) as i32
    }
}
