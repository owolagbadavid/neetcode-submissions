impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut count = HashMap::new();

        let mut res = 0;
        let k = k as usize;
        let mut sum = 0i64;
        let mut l = 0;

        for r in 0..nums.len() {
            *count.entry(nums[r]).or_insert(0) += 1;
            sum += nums[r] as i64;

            while *count.get(&nums[r]).unwrap() > 1 || r-l+1 > k {
                *count.get_mut(&nums[l]).unwrap() -= 1;
                sum -= nums[l] as i64;
                l += 1;
            }

            if r - l + 1 == k {
                res = res.max(sum);
            }
        }

        res
    }
}
