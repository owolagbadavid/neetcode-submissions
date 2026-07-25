impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        let rows = grid.len();
        let cols = grid[0].len();
        let mut start = (0, 0);

        'outer: for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == 1 {
                    start = (i, j);
                    break 'outer;
                }
            }
        }

        let mut q = VecDeque::from([start]);
        let mut visit = vec![vec![false; cols]; rows];
        visit[start.0][start.1] = true;

        while !q.is_empty() {
            while let Some((r, c)) = q.pop_front() {
                for (dr, dc) in &directions {
                    let (nr, nc) = (r as i32 + dr, c as i32 + dc);
                    if nr < 0 || nc < 0 || nr >= rows as i32 || nc >= cols as i32 || grid[nr as usize][nc as usize] == 0 {
                        res += 1;
                    } else if visit[nr as usize][nc as usize] {
                        continue;
                    } else {
                        q.push_back((nr as usize, nc as usize));
                        visit[nr as usize][nc as usize] = true;
                    }
                }
            }
        }

        res
    }
}
