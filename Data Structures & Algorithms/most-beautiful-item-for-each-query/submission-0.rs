impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        items.sort();
        let n = items.len();
        let mut max_b = 0;
        let mut max = vec![0; n];
        for i in 0..n {
            max_b = max_b.max(items[i][1]);
            max[i] = max_b;
        }

        let mut res = vec![];

        for q in queries {
            let mut p = None;
            let (mut l, mut r) = (0, (n-1) as i32);
            while l <= r {
                let m = (r + l) / 2;
                if items[m as usize][0] <= q {
                    p = Some(m);
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }
            if let Some(p_index) = p {
                res.push(max[p_index as usize]);
            } else {
                res.push(0);
            }
        }

        res
    }
}
