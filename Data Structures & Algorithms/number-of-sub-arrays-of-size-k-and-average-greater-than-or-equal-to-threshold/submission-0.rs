impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut l = 0;
        let mut r = (k - 1) as usize;
        let mut res = 0;

        let get_avg = |num: i32| num as f64 / k as f64;

        let mut avg = arr.iter().take(k as usize).fold(0., |a, b| a + get_avg(*b));

        while r < arr.len() {
            if avg as i32 >= threshold {
                res += 1;
            }

            avg -= get_avg(arr[l]);
            if r + 1 < arr.len() {
                avg += get_avg(arr[r+1]);
            }
            r += 1;
            l += 1;
        }   


        res
    }
}
