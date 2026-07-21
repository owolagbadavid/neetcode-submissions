impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let total: i32 = nums.iter().sum();
        let size = total / k;
        if total % k != 0 {
            return false;
        }

        let mut tracker = vec![false; nums.len()];

        fn backtrack(mut k: i32, mut cur: i32, nums: &[i32], tracker: &mut [bool], size: i32) -> bool {
            if cur == size {
                k -= 1;
                cur = 0;
                if k == 0 {
                    return true;
                }
            }

            for i in 0..nums.len() {
                if !tracker[i] && cur + nums[i] <= size {
                    tracker[i] = true;
                    if backtrack(k, cur+nums[i], nums, tracker, size) {
                        return true;
                    }
                    tracker[i] = false;
                }
            }

            false
        }

        backtrack(k, 0, &nums, &mut tracker, size)
    }
}
