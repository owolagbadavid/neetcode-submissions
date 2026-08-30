impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let mut i_x = 0;
        let mut i_n = 0;
        let n = (n - 1) as i64;
        let mut x = x as i64;

        while (n >> i_n) > 0 {
            if (x >> i_x) & 1 == 0 {
                x |= ((n >> i_n) & 1) << i_x;
                i_n += 1;
            }
            i_x += 1;
        }
 
        x
    }
}
