impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut res = 0;
        let mut arr = vec![];

        Self::backtrack(n as usize, n as usize, &mut arr, &mut res);

        res
    }

    fn backtrack(n: usize, left: usize, arr: &mut Vec<Vec<i8>>, res: &mut i32) {
        if left == 0 {
            *res += 1;
            return;
        }

        for i in 0..n {
            let mut cur = vec![0; n];
            cur[i] = 1;
            let row = n - left;

            if arr.iter().enumerate().all(
                |(index, x)| *x != cur && 
                (i < (row-index) || x[i - (row-index)] != 1) && 
                (i + (row-index) >= n || x[i + (row-index)] != 1)
            ) {
                arr.push(cur.clone());
                Self::backtrack(n, left-1, arr, res);
                arr.pop();
            }
        }
    }
}
