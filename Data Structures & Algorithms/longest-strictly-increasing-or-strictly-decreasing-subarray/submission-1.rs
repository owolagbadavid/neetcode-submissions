impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut res = 0;

        for i in 0..nums.len() {
            if !stack.is_empty() && *stack.last().unwrap() >= nums[i] {
                stack = vec![];
            }
            stack.push(nums[i]);
            res = res.max(stack.len() as i32);
        }

        stack = vec![];

        for i in 0..nums.len() {
            if !stack.is_empty() && *stack.last().unwrap() <= nums[i] {
                stack = vec![];
            }
            stack.push(nums[i]);
            res = res.max(stack.len() as i32);
        }

        res
    }
}
