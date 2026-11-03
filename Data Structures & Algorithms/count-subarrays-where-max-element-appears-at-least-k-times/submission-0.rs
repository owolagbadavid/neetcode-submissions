impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let maximum = *nums.iter().max().unwrap();

        let (mut res, mut count, n) = (0, 0, nums.len());
        let mut l = 0;

        for r in 0..n {
            count += if nums[r] == maximum {1} else {0};

            while count == k {
                res += (n - r) as i64;
                count -= if nums[l] == maximum {1} else {0};
                l += 1;
            }
        }

        res
    }
}
