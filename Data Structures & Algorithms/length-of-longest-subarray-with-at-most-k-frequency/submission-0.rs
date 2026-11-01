impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let mut count = HashMap::new();

        let mut l = 0;

        for r in 0..nums.len() {
            *count.entry(nums[r]).or_insert(0) += 1;

            while *count.get(&nums[r]).unwrap() > k {
                *count.get_mut(&nums[l]).unwrap() -= 1;
                l += 1;
            }

            res = res.max(r-l+1);
        }

        res as i32
    }
}
