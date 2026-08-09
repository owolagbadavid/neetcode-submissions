impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        Self::dfs(0, n, &mut HashMap::new())
    }

    fn dfs(sum: i32, n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if sum == n {
            return 0;
        }
        if memo.contains_key(&sum) {
            return memo[&sum];
        }
        let mut i = 1;
        let mut res = i32::MAX;
        while i*i <= n {
            if (sum + i*i) <= n {
                res = min(res, 1 + Self::dfs(sum+i*i, n, memo));
            }
            i += 1;
        }
        memo.insert(sum, res);
        res
    }
}
