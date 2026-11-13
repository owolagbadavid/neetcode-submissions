impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let n = nums.len();
        let mut r = n - 1;

        while l <= r {
            let m = (l+r) / 2;

            if (m == 0 || nums[m] > nums[m-1]) && (m == n-1 || nums[m] > nums[m+1]) {
                return m as i32
            } else if m > 0 && nums[m-1] > nums[m] {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        -1
    }
}
