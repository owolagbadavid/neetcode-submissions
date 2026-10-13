impl Solution {
    pub fn rearrange_array(mut nums: Vec<i32>) -> Vec<i32> {
        let (mut p_i, mut n_i) = (0, 1);
        let mut res = vec![0; nums.len()];

        for i in 0..nums.len() {
            if nums[i] > 0 && p_i < nums.len() {
                res[p_i] = nums[i];
                p_i += 2;
            } else if nums[i] < 0 && n_i < nums.len() {
                res[n_i] = nums[i];
                n_i += 2;
            }
        }

        return res;
    }
}
