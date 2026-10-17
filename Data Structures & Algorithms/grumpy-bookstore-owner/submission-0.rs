impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let sum = customers.iter().zip(&grumpy).filter(|(_, g)| **g == 0).map(|(c, _)| c).sum::<i32>();
        let n = customers.len();
        let mut res = 0;
        let minutes = minutes as usize;

        let mut l = 0;
        let mut cur = 0;

        for r in 0..n {
            if grumpy[r] == 1 {
                cur += customers[r];
            }
            
            if r - l + 1 == minutes {
                res = res.max(cur);
                if grumpy[l] == 1 {
                    cur -= customers[l];
                }
                l += 1;
            }
        }

        sum + res
    }
}
