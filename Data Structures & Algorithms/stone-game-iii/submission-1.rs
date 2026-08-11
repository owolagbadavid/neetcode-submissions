impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {

        fn dfs(i: usize, n: usize, stone_value: &Vec<i32>, cache: &mut [Option<i32>]) -> i32 {
            if i >= n {
                return 0;
            }

            if let Some(res) = cache[i] {
                return res;
            }

            let mut res = i32::MIN;
            
            let mut sum = 0;
            let mut j = i;
            while j < min(i+3, n) {
                sum += stone_value[j];
                res = max(res, sum - dfs(j+1, n, stone_value, cache));
                j += 1;
            }
            cache[i] = Some(res);
            res
        }

        let res = dfs(0, stone_value.len(), &stone_value, &mut vec![None; stone_value.len()]);
        if res == 0 { String::from("Tie") } else if res > 0 { String::from("Alice") } else { String::from("Bob") }
    }
}
