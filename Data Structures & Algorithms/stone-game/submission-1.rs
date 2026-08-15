impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let n = piles.len();
        
        fn dfs(l: usize, r: usize, piles: &Vec<i32>, memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
            if l == r {
                return piles[l];
            }

            if let Some(res) = memo[l][r] {
                return res;
            }
            let res = max(piles[l] - dfs(l+1, r, piles, memo), piles[r] - dfs(l, r-1, piles, memo));
            memo[l][r] = Some(res);
            res
        }

        dfs(0, n-1, &piles, &mut vec![vec![None; n]; n]) > 0
    }
}
