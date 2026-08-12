impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {

        let (r, c) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp = vec![vec![0; c+1]; r+1];
        dp[r-1][c-1] = if obstacle_grid[r-1][c-1] == 1 {0} else {1};
        for i in (0..r-1).rev() {
            if obstacle_grid[i][c-1] != 1 && dp[i+1][c-1] != 0 {
                dp[i][c-1] = dp[r-1][c-1];
            }
        }
        for i in (0..c-1).rev() {
            if obstacle_grid[r-1][i] != 1 && dp[r-1][i+1] != 0 {
                dp[r-1][i] = dp[r-1][c-1];
            }
        }

        if dp[r-1][c-1] == 0 {
            return 0;
        } else {
            for i in (0..r-1).rev() {
                for j in (0..c-1).rev() {
                    if obstacle_grid[i][j] != 1 {
                        dp[i][j] = dp[i+1][j] + dp[i][j+1];
                    }
                }
            }
        }

        dp[0][0]
    }
}
