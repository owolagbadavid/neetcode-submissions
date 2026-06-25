// The guess API is already defined for you.
// fn guess(num: i64) -> i32:
//     -1 if num is higher than the picked number
//      1 if num is lower than the picked number
//      0 if num is equal to the picked number

impl Solution {
    pub unsafe fn guess_number(n: i64) -> i64 {
        let mut l : i64 = 0;
        let mut r : i64 = n;

        while l <= r {
            let m = (l + r) / 2;
            let g = guess(m);
            if g == -1 {
                r = m - 1;
            } else if g == 1 {
                l = m + 1;
            } else {
                return m
            }
        }

        l
    }
}
