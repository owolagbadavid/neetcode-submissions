impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();

        let mut l = 0;
        let mut total = 0;
        let mut res = 1;

        for r in 0..nums.len() {
            total += nums[r];
            while nums[r] * (r-l+1) as i32 > total + k {
                total -= nums[l];
                l += 1;
            }

            res = res.max((r-l+1) as i32);
        }


        res
    }
}
