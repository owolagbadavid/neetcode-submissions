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
                    start = (i as i32, j as i32);
                    break 'outer;
                }
            }
        }

        let mut q = VecDeque::from([start]);
        let mut visit = HashSet::from([start]);

        while !q.is_empty() {
            for i in 0..q.len() {
                let (r, c) = q.pop_front().unwrap();
                for (dr, dc) in &directions {
                    let (nr, nc) = (r+dr, c+dc);
                    if visit.contains(&(nr, nc)) {
                        continue;
                    } else if nr < 0 || nc < 0 || nr >= rows as i32 || nc >= cols as i32 || grid[nr as usize][nc as usize] == 0 {
                        res += 1;
                    } else {
                        q.push_back((nr, nc));
                        visit.insert((nr, nc));
                    }
                }
            }
        }

        res
    }
}
