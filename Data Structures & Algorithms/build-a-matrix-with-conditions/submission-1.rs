impl Solution {
    pub fn build_matrix(k: i32, row_conditions: Vec<Vec<i32>>, col_conditions: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut row_adj: HashMap<i32, Vec<i32>> = (1..=k).map(|n| (n, vec![])).collect();
        let mut col_adj: HashMap<i32, Vec<i32>> = (1..=k).map(|n| (n, vec![])).collect();
        let mut res = vec![vec![0; k as usize]; k as usize];

        for cond in row_conditions {
            let (above, below) = (cond[0], cond[1]);
            row_adj.get_mut(&above).unwrap().push(below);
        }

        for cond in col_conditions {
            let (left, right) = (cond[0], cond[1]);
            col_adj.get_mut(&left).unwrap().push(right);
        }

        let (sort_r, r_cycle) = Self::top_sort(&row_adj, k);
        let (sort_c, c_cycle) = Self::top_sort(&col_adj, k);

        if r_cycle || c_cycle {
            return vec![];
        }

        let mut map_index: HashMap<i32, [usize; 2]> = (1..=k).map(|n| (n, [0, 0])).collect();

        for i in 0..(k as usize) {
            let r = sort_r[i];
            map_index.get_mut(&r).unwrap()[0] = i;
            let c = sort_c[i];
            map_index.get_mut(&c).unwrap()[1] = i;
        }

        for (key, value) in &map_index {
            res[value[0]][value[1]] = *key;
        }

        res
    }

    fn top_sort(adj: &HashMap<i32, Vec<i32>>, k: i32) -> (Vec<i32>, bool) {
        let mut visit = vec![false; k as usize];
        let mut res = vec![];

        for i in 1..=k {
            if dfs(adj, i, &mut res, &mut visit, &mut vec![false; k as usize]) {
                return (vec![], true);
            }
        }

        fn dfs(adj: &HashMap<i32, Vec<i32>>, n: i32, res: &mut Vec<i32>, visit: &mut Vec<bool>, cycle: &mut Vec<bool>) -> bool {
            if cycle[(n-1) as usize] {
                return true;
            }

            if visit[(n-1) as usize] {
                return false;
            }

            visit[(n-1) as usize] = true;
            cycle[(n-1) as usize] = true;

            for nei in adj.get(&n).unwrap() {
                if dfs(adj, *nei, res, visit, cycle) {
                    return true;
                }
            }
            res.push(n);
            cycle[(n-1) as usize] = false;
            false
        }

        res.reverse();
        (res, false)
    }
}
