impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let num_len = nums.len();
        let set: HashSet<i32> = nums.into_iter().collect();
        num_len != set.len()
    }
}
