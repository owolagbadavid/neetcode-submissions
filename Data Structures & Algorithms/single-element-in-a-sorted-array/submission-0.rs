impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let n = nums.len();
        let mut r = n - 1;

        while l <= r {
            let m = (l + r) / 2;
            let mut m2 = m;
            if m < r && nums[m] == nums[m+1] {
                m2 = m + 1;
            } else if m > l && nums[m] == nums[m-1] {
                m2 = m - 1;
            }

            if m == m2 {
                return nums[m];
            }

            if (r - m.max(m2)) % 2 == 0 {
                r = m.min(m2) - 1;
            } else {
                l = m.max(m2) + 1;
            }
        }

        return -1;
    }
}
