impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let sum: i32 = stones.iter().sum();
        let target = sum / 2;

        fn dfs(i: usize, n: usize, target: i32, sum: i32, stones: &Vec<i32>, dp: &mut Vec<Vec<Option<i32>>>) -> i32 {
            if i >= n {
                return sum;
            }

            if let Some(res) = dp[i][sum as usize] {
                return res
            }

            let mut res = dfs(i+1, n, target, sum, stones, dp);

            if sum < target {
                let res2 = dfs(i+1, n, target, sum+stones[i], stones, dp);
                if (target - res).abs() > (target - res2).abs() {
                    res = res2;
                }
            }

            dp[i][sum as usize] = Some(res);
            res
        }
        let other = dfs(0, n, target, 0, &stones, &mut vec![vec![None; (sum + 1) as usize]; n + 1]);

        (sum - other - other).abs()
    }
}
