impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut l = 0;
        let mut r = k as usize;
        let target = threshold * k;
        let mut res = 0;
        let mut sum = arr.iter().take(k as usize).sum::<i32>();

        if sum >= target {
            res += 1;
        }

        while r < arr.len() {
            sum += arr[r] - arr[l];
            if sum >= target {
                res += 1;
            }
            r += 1;
            l += 1;
        }   


        res
    }
}
