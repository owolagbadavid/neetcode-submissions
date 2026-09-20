impl Solution {
    pub fn find_missing_and_repeated_values(mut grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![0, 0];
        let n = grid.len();
        for i in 0..n {
            for j in 0..n {
                let num = (grid[i][j].abs() - 1) as usize;
                let (r, c) = (num / n, num % n);
                if grid[r][c] < 0 {
                    res[0] = (num + 1) as i32;
                }
                grid[r][c] = -grid[r][c].abs();
            }
        }

        let mut count = 0;
        for i in 0..n {
            for j in 0..n {
                count += 1;
                if grid[i][j] > 0 {
                    res[1] = count;
                } 
            }
        }

        res
    }
}
