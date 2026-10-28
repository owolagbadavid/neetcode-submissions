impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let at_most = |target| {
            let mut count = [0, 0];
            let mut res = 0;
            let mut l = 0;

            for r in 0..nums.len() {
                count[(nums[r] % 2) as usize] += 1;

                while count[1] > target {
                    count[(nums[l] % 2) as usize] -= 1;
                    l += 1;
                }

                res += r - l + 1;
            }

            res as i32
        };

        

        at_most(k) - at_most(k-1)
    }
}
