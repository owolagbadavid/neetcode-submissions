impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let dir = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let row = heights.len() as i32;
        let col = heights[0].len() as i32;

        let mut heap = BinaryHeap::from([(0, [0, 0], 0)]);
        let mut visit = HashSet::new();

        while let Some((total, coord, e)) = heap.pop() {
            if visit.contains(&coord) {
                continue;
            }
            visit.insert(coord);
            let [r, c] = coord;
            if r == row - 1 && c == col - 1 {
                return -total;
            }
            for (dr, dc) in &dir {
                let (nr, nc) = (r+dr, c+dc);
                if nr < row && nc < col && min(nr, nc) >= 0 && !visit.contains(&[nr, nc]) {
                    let effort = (heights[nr as usize][nc as usize] - heights[r as usize][c as usize]).abs();
                    heap.push((-max(effort, -total), [nr, nc], effort));
                }
            }
        }

        -1
    }
}
