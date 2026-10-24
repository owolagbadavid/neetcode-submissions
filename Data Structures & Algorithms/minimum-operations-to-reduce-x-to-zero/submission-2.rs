impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, x: i32) -> i32 {
        let n = nums.len();
        let sum = nums.iter().sum::<i32>();
        nums.push(0);
        let mut y = 0;
        let mut res = None;

        let mut l = 0;

        for r in 0..=n {
            if y == sum - x {
                res = Some(res.unwrap_or(i32::MAX).min((n - (r-l)) as i32));
            }

            y += nums[r];

            while l <= r && y > sum - x {
                y -= nums[l];
                l += 1;
            }
        }

        res.unwrap_or(-1)
    }
}
