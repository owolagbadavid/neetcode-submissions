impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut last: Option<i32> = None;
        let mut res = 0;
        let mut count = 0;

        for i in 0..nums.len() {
            if last.is_some() && last.unwrap() >= nums[i] {
                count = 0;
            }
            count += 1;
            last = Some(nums[i]);
            res = res.max(count);
        }

        count = 0;

        for i in 0..nums.len() {
            if  last.is_some() && last.unwrap() <= nums[i] {
                count = 0;
            }
            count += 1;
            last = Some(nums[i]);
            res = res.max(count);
        }

        res
    }
}
