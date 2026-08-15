impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let n = piles.len();
        
        fn dfs(l: isize, r: isize, piles: &Vec<i32>, memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
            if l > r {
                return 0;
            }

            if let Some(res) = memo[l as usize][r as usize] {
                return res;
            }
            let res = max(piles[l as usize] - dfs(l+1, r, piles, memo), piles[r as usize] - dfs(l, r-1, piles, memo));
            memo[l as usize][r as usize] = Some(res);
            res
        }

        dfs(0, (n-1) as isize, &piles, &mut vec![vec![None; n]; n]) > 0
    }
}
