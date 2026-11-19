impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut l = 1;
        let mut r = *quantities.iter().max().unwrap();

        let is_valid = |m| {
            let mut n = n;
            let mut i = 0;
            while i < quantities.len() {
                let mut product = quantities[i];
                n -= product / m;
                n -= if product % m > 0 {1} else {0};
                i += 1;
                if n < 0 {
                    return false;
                }
            }
            n >= 0
        };

        let mut res = r;

        while l <= r {
            let m = (l + r) / 2;

            if is_valid(m) {
                res = res.min(m);
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        res
    }
}
