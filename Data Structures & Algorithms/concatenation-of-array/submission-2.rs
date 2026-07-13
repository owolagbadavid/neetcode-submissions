impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![0; n*2];
        for i in 0..n {
            res[i] = nums[i];
            res[i+n] = nums[i];
        }
        res 
    }
}
