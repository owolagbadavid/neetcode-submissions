impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (r, c) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![i32::MAX; c+1]; r+1];

        dp[r-1][c] = 0;

        for i in (0..r).rev() {
            for j in (0..c).rev() {
                dp[i][j] = grid[i][j] + min(dp[i+1][j], dp[i][j+1])
            }
        }

        dp[0][0]
    }
}
