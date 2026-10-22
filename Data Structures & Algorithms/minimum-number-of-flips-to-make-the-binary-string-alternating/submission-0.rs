impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();

        let mut mis = 0;
        let mut res = usize::MAX;

        for i in 0..2*n {
            if (s[i%n] - b'0') as usize != i%2 {
                mis += 1;
            }

            if i >= n {
                let l = i - n;
                if (s[l%n] - b'0') as usize != l%2 {
                    mis -= 1;
                }
            }

            if i >= n - 1 {
                res = res.min(mis).min(n-mis);
            }
        }

        res as i32
    }
}
