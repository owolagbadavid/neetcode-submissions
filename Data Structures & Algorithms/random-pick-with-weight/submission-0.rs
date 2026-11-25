struct Solution {
    prefix: Vec<i32>
}

impl Solution {
    pub fn new(w: Vec<i32>) -> Self {
        let mut prefix = vec![];
        let mut cur = 0;
        for e in w {
            cur += e;
            prefix.push(cur);
        }

        Self { prefix }
    }

    pub fn pick_index(&self) -> i32 {
        let n = *self.prefix.last().unwrap() as f64 * rand::random::<f64>();

        let mut l = 0;
        let mut r = self.prefix.len();
        while l < r {
            let m = (l + r) / 2;

            if self.prefix[m] as f64 > n {
                r = m;
            } else {
                l = m + 1;
            }
        }

        r as i32
    }
}

// Your Solution object will be instantiated and called as such:
// let obj = Solution::new(w);
// let ret = obj.pick_index();
