impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;

        let mut res = vec![-1; n-k+1];

        let mut l = 0;

        for r in 0..n {
            if r != l && nums[r-1] != nums[r] - 1 {
                l = r;
            }

            if r-l+1 == k {
                res[l] = nums[r];
                l += 1;
            }
        }
        res
    }
}
