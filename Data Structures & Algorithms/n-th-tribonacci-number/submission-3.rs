impl Solution {

    pub fn tribonacci(n: i32) -> i32 {
        let mut dp = [0, 1, 1];
        for i in 3..(n+1) as usize {
            let tmp1 = dp[2];
            let tmp2 = dp[1];
            dp[2] = tmp1 + tmp2 + dp[0];
            dp[1] = tmp1;
            dp[0] = tmp2;
        }
        dp[min(2, n) as usize]
    }
}
