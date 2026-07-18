impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut arr = vec![];

        Self::backtrack(1, n, k, &mut res, &mut arr);

        res
    }

    fn backtrack(i: i32, n: i32, k: i32, res: &mut Vec<Vec<i32>>, arr: &mut Vec<i32>) {
        if i > n {
            if arr.len() as i32 == k {
                res.push(arr.clone());
            }
            return;
        }

        Self::backtrack(i+1, n, k, res, arr);

        arr.push(i);
        Self::backtrack(i+1, n, k, res, arr);
        arr.pop();
    }
}
