impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;

        let mut l = 0;
        let mut used = 0;

        for r in 0..nums.len() {
            used += if nums[r] == 0 {1} else {0};

            while used > k {
                used -= if nums[l] == 0 {1} else {0};
                l += 1;
            }

            res = res.max(r+1-l);
        }

        res as i32
    }
}
