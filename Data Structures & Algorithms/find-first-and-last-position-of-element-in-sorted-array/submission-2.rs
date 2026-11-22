impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res = vec![-1, -1];
        let n = nums.len();

        if n == 0 {
            return res;
        }

        let mut l = 0;
        let mut r = nums.len() - 1;

        while l <= r {
            let m = (r + l) / 2;
            if nums[m] < target {
                l = m + 1;
            } else if nums[m] > target {
                if m == 0 { break; }
                r = m - 1;
            } else {
                res[0] = m as i32;
                if m == 0 { break; }
                r = m - 1;
            }
        }

        (l, r) = (0, nums.len() - 1);

        while l <= r {
            let m = (r + l) / 2;
            if nums[m] > target {
                if m == 0 { break; }
                r = m - 1;
            } else if nums[m] < target {
                l = m + 1;
            } else {
                res[1] = m as i32;
                l = m + 1;
            }
        }

        res
    }
}
