impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut arr = vec![1; n+1];
        for i in 2..n+1 {
            arr[i] = arr[i-1] + arr[i-2];
        }
        arr[n]
    }
}
