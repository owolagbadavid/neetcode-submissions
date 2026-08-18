impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let mut res = 1;
        let mut last: Option<bool> = None;
        let mut l = 0;
        for r in 1..arr.len() {
            let next = if arr[r-1] > arr[r] { Some(true) } else if arr[r-1] < arr[r] { Some(false) } else { None };
            if next.is_none() || last == next {
                l = if next.is_none() {r} else {r-1};
            }

            last = next;
            res = res.max((r-l+1) as i32);
        }

        res
    }
}
