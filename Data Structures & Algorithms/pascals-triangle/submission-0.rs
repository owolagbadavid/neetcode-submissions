impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![1]];

        for i in 2..=(num_rows as usize) {
            let mut cur = vec![1; i];

            for j in 1..i-1 {
                cur[j] = res[i-2][j] + res[i-2][j-1];
            }

            res.push(cur);
        }

        res
    }
}
