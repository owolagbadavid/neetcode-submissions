impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        fn dfs(piles: &Vec<i32>, i: usize, bit: usize, n: usize, m: usize, cache: &mut HashMap<(usize, usize, usize), i32>) -> i32 {
            if i >= n {
                return 0;
            }

            if cache.contains_key(&(bit, i, m)) {
               return cache[&(bit, i, m)];
            }

            let mut res = 0;
            if bit == 1 {
                res = i32::MAX;
            }
            let mut sum = 0;
            let end = min(n, i + 2*m);
            for j in i..end {
                sum += piles[j];
                if bit == 0 {
                    res = max(res, sum + dfs(piles, j+1, 1, n, max(m, j-i+1), cache));
                } else {
                    res = min(res, dfs(piles, j+1, 0, n, max(m, j-i+1), cache));
                }
            }

            cache.insert((bit, i, m), res);

            res
        }

        dfs(&piles, 0, 0, piles.len(), 1, &mut HashMap::new())
    }
}
