impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let mut dp = vec![1; n as usize];

        for num in 2..=n {
            dp[(num-1) as usize] = if num == n {0} else {num};
            for k in 1..num {
                let cur = dp[(num-1) as usize];
                let l = dp[(k-1) as usize];
                let r = dp[(num-k-1) as usize];
                dp[(num-1) as usize] = max(cur, l * r)
            }
        }

        dp[(n-1) as usize]
    }
}
