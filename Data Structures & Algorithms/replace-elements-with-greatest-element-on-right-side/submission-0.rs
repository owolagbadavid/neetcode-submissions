impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len();
        let mut max = arr[n-1];
        arr[n-1] = -1;

        for i in (0..n-1).rev() {
            let cur = max;
            max = max.max(arr[i]);
            arr[i] = cur;
        }

        arr
    }
}
