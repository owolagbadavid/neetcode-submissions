impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        Self::dfs(0, n, &mut vec![-1; n as usize])
    }

    fn dfs(sum: i32, n: i32, memo: &mut Vec<i32>) -> i32 {
        if sum == n {
            return 0;
        }
        if memo[sum as usize] != -1 {
            return memo[sum as usize];
        }
        let mut i = 1;
        let mut res = i32::MAX;
        while i*i <= n - sum {
            res = min(res, 1 + Self::dfs(sum+i*i, n, memo));
            i += 1;
        }
        memo[sum as usize] = res;
        res
    }
}
