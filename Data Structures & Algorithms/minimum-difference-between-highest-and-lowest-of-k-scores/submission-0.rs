impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut res = i32::MAX;

        let mut l = 0;
        let mut r = (k - 1) as usize;

        while r < nums.len() {
            res = res.min(nums[r] - nums[l]);
            r += 1;
            l += 1;
        }

        res
    }
}
