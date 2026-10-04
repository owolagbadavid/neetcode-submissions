impl Solution {
    pub fn max_product_difference(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let n = nums.len();

        (nums[n-2] * nums[n-1]) - (nums[0] * nums[1])
    }
}