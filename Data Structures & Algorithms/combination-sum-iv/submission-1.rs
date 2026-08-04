impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        Self::dfs(0, target, &nums, &mut HashMap::new())
    }

    fn dfs(cur: i32, target: i32, nums: &Vec<i32>, cache: &mut HashMap<i32, i32>) -> i32 {
        if cur == target {
            return 1;
        }

        if cache.contains_key(&cur) {
            return cache[&cur]
        }

        let mut res = 0;

        for j in 0..nums.len() {
            if cur+nums[j] <= target {
                res += Self::dfs(cur+nums[j], target, nums, cache);
            }
        }

        cache.insert(cur, res);

        res
    }
}
