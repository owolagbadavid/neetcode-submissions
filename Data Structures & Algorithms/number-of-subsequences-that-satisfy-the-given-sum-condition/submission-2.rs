impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut res = 0;
        nums.sort();
        let m = 10i64.pow(9) + 7;
        let mut r = nums.len() - 1;
        let mut pow = vec![1i64; nums.len()];
        for k in 1..nums.len() {
            pow[k] = pow[k-1] * 2 % m;
        }

        for i in 0..nums.len() {
            while nums[i] + nums[r] > target && i < r {
                r -= 1;
            }

            if i < r || nums[i] + nums[r] <= target {
                res += pow[r-i];
                res %= m;
            }
        }

        res as i32
    }
}
