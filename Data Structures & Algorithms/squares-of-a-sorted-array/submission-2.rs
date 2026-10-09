impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![0; n];

        let (mut l, mut r, mut i) = (1, n, n);

        while l <= r {
            if nums[l-1].abs() > nums[r-1].abs() {
                res[i-1] = nums[l-1].abs().pow(2);
                l += 1;
            } else {
                res[i-1] = nums[r-1].abs().pow(2);
                r -= 1;
            }
            i -= 1;
        }

        res
    }
}
