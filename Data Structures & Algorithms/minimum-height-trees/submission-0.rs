impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj: HashMap<i32, Vec<i32>> = (0..n).map(|i| (i, vec![])).collect();
        let mut min = n;
        let mut res = vec![];

        for edge in &edges {
            adj.get_mut(&edge[0]).unwrap().push(edge[1]);
            adj.get_mut(&edge[1]).unwrap().push(edge[0]);
        }

        for i in 0..n {
            let mut q = VecDeque::from([i]);
            let mut visit = HashSet::from([i]);
            let mut cur = 0;

            while !q.is_empty() {
                for _ in 0..q.len() {
                    let node = q.pop_front().unwrap();

                    for n in adj.get(&node).unwrap() {
                        if visit.insert(*n) {
                            q.push_back(*n);
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
