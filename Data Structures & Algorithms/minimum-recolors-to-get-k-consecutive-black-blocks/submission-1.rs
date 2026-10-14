impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let blocks = blocks.as_bytes();
        let mut count = 0;
        let mut res = 100;

        let mut l = 0;
        for r in 0..blocks.len() {
            if blocks[r] == b'B' {
                count += 1;
            }

            if r - l + 1 == k as usize {
                res = res.min(k - count);
                if blocks[l] == b'B' {
                    count -= 1;
                }
                l += 1;
            }
        }

        res
    }
}
