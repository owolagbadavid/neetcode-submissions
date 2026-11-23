impl Solution {
    pub fn maximum_removals(s: String, p: String, removable: Vec<i32>) -> i32 {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let is_sub = |removed: HashSet<usize>| {
            let (mut i, mut j) = (0, 0);
            while i < s.len() && j < p.len() {
                if !removed.contains(&i) && p[j] == s[i] {
                    i += 1;
                    j += 1;
                } else {
                    i += 1;
                }
            }

            j >= p.len()
        };

        let mut res = 0;
        let (mut l, mut r) = (0, (removable.len()-1) as i32);

        while l <= r {
            let m = (l + r) / 2;

            let removed: HashSet<usize> = removable[0..=m as usize].iter().map(|e| *e as usize).collect();
            if is_sub(removed) {
                res = res.max(m+1);
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        res as i32
    }
}
