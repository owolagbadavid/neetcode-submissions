impl Solution {
    pub fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (r, c) = (obstacle_grid.len(), obstacle_grid[0].len());

        fn dfs(i: usize, j: usize, r: usize, c: usize, obstacle_grid: &mut Vec<Vec<i32>>, cache: &mut Vec<Vec<Option<i32>>>) -> i32 {
            if i >= r || j >= c || obstacle_grid[i][j] == 1 {
                return 0;
            }

            if i == r-1 && j == c-1 {
                return 1;
            }

            if let Some(res) = cache[i][j] {
                return res;
            }

            cache[i][j] = Some(dfs(i+1, j, r, c, obstacle_grid, cache) + dfs(i, j+1, r, c, obstacle_grid, cache));
            cache[i][j].unwrap()
        }

        dfs(0, 0, r, c, &mut obstacle_grid, &mut vec![vec![None; c]; r])
    }
}
