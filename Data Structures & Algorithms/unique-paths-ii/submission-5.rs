impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {

        let (r, c) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp = vec![0; c+1];
        dp[c-1] = 1;
     

        for i in (0..r).rev() {
            for j in (0..c).rev() {
                if obstacle_grid[i][j] == 1 {
                    dp[j] = 0;
                } else {
                    dp[j] += dp[j+1];
                }
            }
        }

        dp[0]
    }
}
