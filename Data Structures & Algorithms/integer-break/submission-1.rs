impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        Self::dfs(n, n, &mut HashMap::from([(1,1)]))
    }

    fn dfs(n: i32, num: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        if cache.contains_key(&num) {
            return cache[&num];
        }

        let mut res = if num == n {0} else {num};
        for k in 1..num {
            res = max(res, Self::dfs(n, k, cache) * Self::dfs(n, num - k, cache));
        }
        cache.insert(num, res);
        res
    }
}
