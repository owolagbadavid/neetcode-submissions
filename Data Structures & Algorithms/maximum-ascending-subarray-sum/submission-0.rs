impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut res = 0;
        let mut last = 0;

        for num in nums {
            if num <= last {
                sum = num;
            } else {
                sum += num;
            }
            last = num;
            res = res.max(sum);
        }

        res
    }
}
