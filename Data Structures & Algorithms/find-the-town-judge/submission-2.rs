impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut adj: HashMap<i32, i32> = (1..=n).map(|e| (e, 0)).collect();

        for pair in &trust {
            *adj.get_mut(&pair[1]).unwrap() += 1;
            *adj.get_mut(&pair[0]).unwrap() -= 1;
        }

        for i in 1..=n {
            if adj[&i] == n - 1 {
                return i;
            }
        }

        -1
    }
}
