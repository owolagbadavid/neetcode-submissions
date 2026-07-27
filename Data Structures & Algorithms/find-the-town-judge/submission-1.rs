impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut adj: HashMap<i32, HashSet<i32>> = (1..=n).map(|e| (e, HashSet::new())).collect();
        let mut trust_none: HashSet<i32> = (1..=n).collect();

        for pair in &trust {
            let list = adj.get_mut(&pair[0]).unwrap();
            list.insert(pair[1]);
            trust_none.remove(&pair[0]);
        }

        if trust_none.len() == 1 {
            let candidate = trust_none.into_iter().next().unwrap();
            for i in 1..=n {
                if i == candidate {
                    continue;
                }

                if !adj[&i].contains(&candidate) {
                    return -1;
                }
            }
            return candidate;
        }

        -1
    }
}
