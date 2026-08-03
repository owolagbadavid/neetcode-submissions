impl Solution {

    pub fn tribonacci(n: i32) -> i32 {
        Self::dfs(&mut HashMap::new(), n)
    }

    pub fn dfs(cache: &mut HashMap<i32, i32>, n: i32) -> i32 {
        if cache.contains_key(&n) {
            return cache[&n];
        }
        if n < 3 {
            return if n == 0 {0} else {1};
        }
        let result = Self::dfs(cache, n-1) + Self::dfs(cache, n-2) + Self::dfs(cache, n-3);
        cache.insert(n, result);
        result
    }
}
