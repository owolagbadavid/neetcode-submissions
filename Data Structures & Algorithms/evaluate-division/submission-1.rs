use std::collections::HashMap;

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut divs = HashMap::new();
        let mut ans = vec![-1.; queries.len()];

        for i in 0..values.len() {
            divs.entry(&equations[i][0]).or_insert(vec![]).push((&equations[i][1], values[i]));
            divs.entry(&equations[i][1]).or_insert(vec![]).push((&equations[i][0], 1./values[i]));
        }

        for (i, query) in queries.into_iter().enumerate() {
            if let [src, dest] = query.as_slice() {
                let mut q = VecDeque::from([(src, 1.)]);
                let mut visit = HashSet::from([src]);

                'outer: while let Some((cur, val)) = q.pop_front() {
                    if let Some(list) = divs.get(cur) {
                        for (next, res) in list {
                            if next == &dest {
                                ans[i] = res*val;
                                break 'outer;
                            } else if visit.insert(next) {
                                q.push_back((next, res*val));
                            }
                        }
                    } else {
                        break;
                    }
                } 
            }
        }

        ans
    }
}
