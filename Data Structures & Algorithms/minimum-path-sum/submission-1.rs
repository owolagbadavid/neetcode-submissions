impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (r, c) = (grid.len(), grid[0].len());

        fn dfs(i :usize, j: usize, r: usize, c: usize, grid: &Vec<Vec<i32>>, memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
            if i >= r || j >= c {
                return 201;
            }

            if let Some(res) = memo[i][j] {
                return res;
            }

            if i == r-1 && j == c-1 {
                return grid[i][j];
            }

            memo[i][j] = Some(grid[i][j] + min(dfs(i+1, j, r, c, grid, memo), dfs(i, j+1, r, c, grid, memo)));
            memo[i][j].unwrap()
        }

        dfs(0, 0, r, c, &grid, &mut vec![vec![None; c]; r])
    }
}
