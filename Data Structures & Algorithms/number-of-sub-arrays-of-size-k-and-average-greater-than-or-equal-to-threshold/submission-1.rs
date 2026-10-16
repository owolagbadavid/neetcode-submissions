impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut l = 0;
        let mut r = (k - 1) as usize;
        let mut res = 0;

        let mut sum = arr.iter().take(k as usize).sum::<i32>();
        while r < arr.len() {
            if sum/k >= threshold {
                res += 1;
            }

            sum -= arr[l];
            if r + 1 < arr.len() {
                sum += arr[r+1];
            }
            r += 1;
            l += 1;
        }   


        res
    }
}
