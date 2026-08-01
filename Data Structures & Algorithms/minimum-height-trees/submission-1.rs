impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj = vec![vec![]; n as usize];
        let mut res = vec![];
        let mut min = n;

        for edge in &edges {
            adj[edge[0] as usize].push(edge[1]);
            adj[edge[1] as usize].push(edge[0]);
        }

        for i in 0..n {
            let mut q = VecDeque::from([i]);
            let mut visit = vec![false; n as usize];
            visit[i as usize] = true;
            let mut cur = 0;

            while !q.is_empty() {
                for _ in 0..q.len() {
                    let node = q.pop_front().unwrap();

                    for next in &adj[node as usize] {
                        if !visit[*next as usize] {
                            q.push_back(*next);
                            visit[*next as usize] = true;
                        }
                    }
                }
                cur += 1
            }

            if cur < min {
                min = cur;
                res.clear();
                res.push(i);
            } else if cur == min {
                res.push(i);
            }
        }

        res
    }
}
