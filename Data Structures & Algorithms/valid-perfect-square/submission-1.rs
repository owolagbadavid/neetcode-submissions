impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let num = num as i64;
        let mut l = 1;
        let mut r = num;

        while l <= r {
            let m = (l + r) / 2;
            if m*m > num {
                r = m - 1;
            } else if m*m < num {
                l = m + 1;
            } else {
                return true;
            }
        } 

        false
    }
}
