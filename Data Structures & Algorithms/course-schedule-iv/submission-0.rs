impl Solution {
    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut adj: HashMap<i32, Vec<i32>> = (0..num_courses).map(|n| (n, vec![])).collect();
        let mut res = vec![false; queries.len()];
        for i in 0..prerequisites.len() {
            let pair = &prerequisites[i];
            let list = &mut adj.get_mut(&pair[0]).unwrap();
            list.push(pair[1]);
        }

        for i in 0..queries.len() {
            let mut q = VecDeque::from([queries[i][0]]);
            let mut visit = HashSet::new();
            
            while let Some(cur) = q.pop_front() {
                if cur == queries[i][1] {
                    res[i] = true;
                    q.clear();
                    visit.clear();
                }

                for edge in &adj[&cur] {
                    if visit.insert(*edge) {
                        q.push_back(*edge);
                    }
                }
            }
        }

        res
    }
}
