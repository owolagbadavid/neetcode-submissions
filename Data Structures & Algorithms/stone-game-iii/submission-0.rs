impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {

        fn dfs(i: usize, n: usize, stone_value: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
            if i >= n {
                return 0;
            }

            if cache[i] != i32::MIN {
                return cache[i];
            }

            let mut res = i32::MIN;
            
            let mut sum = 0;
            let mut j = i;
            while j < min(i+3, n) {
                sum += stone_value[j];
                res = max(res, sum - dfs(j+1, n, stone_value, cache));
                j += 1;
            }
            cache[i] = res;
            cache[i]
        }

        let res = dfs(0, stone_value.len(), &stone_value, &mut vec![i32::MIN; stone_value.len()]);
        if res == 0 { String::from("Tie") } else if res > 0 { String::from("Alice") } else { String::from("Bob") }
    }
}
