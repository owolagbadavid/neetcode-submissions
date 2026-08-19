impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {

        fn dfs(i: usize, chars: &[char], min_jump: usize, max_jump: usize, memo: &mut [Option<bool>]) -> bool {
            if i == chars.len() - 1 {
                return true;
            }

            if i >= chars.len() {
                return false;
            }

            if let Some(res) = memo[i] {
                return res;
            }

            let mut res = false;
            for j in min_jump..=max_jump {
                if i+j < chars.len() && chars[i+j] == '0' {
                    res = res || dfs(i+j, chars, min_jump, max_jump, memo);
                }
            }

            memo[i] = Some(res);

            res
        }
        dfs(0, &s.chars().collect::<Vec<char>>(), min_jump as usize, max_jump as usize, &mut vec![None; s.len()])
    }
}
