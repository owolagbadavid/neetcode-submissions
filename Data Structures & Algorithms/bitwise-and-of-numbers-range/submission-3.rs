impl Solution {
    pub fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
        let mut res = 0;

        for i in 0..32 {
            let bit = (left >> i) & 1;
            if bit == 0 {
                continue;
            }

            let diff = (1 << (i+1)) - (left % (1 << (i+1)));

            if right - left < diff {
                res = res | (1 << i);
            }
        }

        res
    }
}
