impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res = vec![-1, -1];
        let n = nums.len() as i32;

        if n == 0 {
            return res;
        }

        let (mut l, mut r) = (0i32, n - 1);

        while l <= r {
            let m = (r + l) / 2;
            if nums[m as usize] < target {
                l = m + 1;
            } else if nums[m as usize] > target {
                r = m - 1;
            } else {
                res[0] = m;
                r = m - 1;
            }
        }

        (l, r) = (0, n - 1);
        while l <= r {
            let m = (r + l) / 2;
            if nums[m as usize] > target {
                r = m - 1;
            } else if nums[m as usize] < target {
                l = m + 1;
            } else {
                res[1] = m ;
                l = m + 1;
            }
        }

        res
    }
}
