impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        let n = matchsticks.len();
        matchsticks.sort();
        matchsticks.reverse();
        let perimeter = matchsticks.iter().sum();
        let length = perimeter / 4;
        let mut checker = vec![false; n];

        if matchsticks[0] > length ||perimeter % 4 != 0 {
            return false;
        }

        return Self::backtrack(length, n, 0, &mut checker, &matchsticks, perimeter);
    }

    fn backtrack(l: i32, n: usize, mut size: i32, checker: &mut [bool], matchsticks: &Vec<i32>, left: i32) -> bool {
        if size == l {
            size = 0;
            if left == 0 {
                return true;
            }
        }

        let mut res = false;
        for i in 0..n {
            if i > 0 && matchsticks[i] == matchsticks[i-1] && !checker[i-1] {
                continue;
            }
            if !checker[i] && size + matchsticks[i] <= l {
                checker[i] = true;
                if  Self::backtrack(l, n, size + matchsticks[i], checker, matchsticks, left - matchsticks[i]) {
                    return true;
                }
                checker[i] = false;
            }
        }

        res
    }
}
