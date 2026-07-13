impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            let key = target-nums[i];
            if map.contains_key(&key) {
                return vec![map[&key], i as i32];
            } else {
                map.insert(nums[i], i as i32);
            }
        }
        return vec![];
    }
}
