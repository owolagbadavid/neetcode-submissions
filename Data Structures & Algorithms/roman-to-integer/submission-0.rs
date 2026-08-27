impl Solution {
    pub fn roman_to_int(mut s: String) -> i32 {
        let map = HashMap::from([(b'I', 1), (b'V', 5), (b'X', 10), (b'L', 50), (b'C', 100), (b'D', 500), (b'M', 1000)]);

        s.push('I');
        let mut s = s.as_bytes();
        let n = s.len() - 1;

        let mut res = 0;

        for i in 0..n {
            let cur = map[&s[i]];
            let next = map[&s[i+1]];

            if cur < next {
                res -= cur;
            } else {
                res += cur;
            }
        }

        res
    }
}
