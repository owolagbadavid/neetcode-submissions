impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let n = chars.len();
        let mut l = 0;
        let mut res = 0;

        for r in 0..n {
            if r + 1 >= n || chars[r+1] != chars[r] {
                chars[res] = chars[r];
                res += 1;
                let mut start = res;
                let mut len = r - l + 1;
                while r != l && len > 0 {
                    chars[res] = char::from_digit((len % 10) as u32, 10).unwrap();
                    len /= 10;
                    res += 1;
                }
                let mut end = res - 1;
                while start < end {
                    chars.swap(start, end);
                    start += 1;
                    end -= 1;
                }
                l = r + 1;
            }
        }

        res as i32
    }
}
