impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let mut total = [0, 0, 0];
        let s = s.as_bytes();
        let n = s.len();
        let mut res = 0;

        for i in 0..n {
            total[(s[i] - b'a') as usize] += 1;
        }

        if total.iter().any(|&c| c < k) {
            return -1;
        }

        let mut l = 0;
        let mut cur = [0, 0, 0];

        for r in 0..n {
            total[(s[r] - b'a') as usize] -= 1;

            let diff: [i32; 3] = std::array::from_fn(|i| total[i] - cur[i]);

            while total[(s[r] - b'a') as usize] < k {
                total[(s[l] - b'a') as usize] += 1;
                l += 1;
            }

            res = res.max(r-l+1);
        }

        (n - res) as i32
    }
}
