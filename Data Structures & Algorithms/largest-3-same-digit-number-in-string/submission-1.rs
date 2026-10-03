impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let num = num.as_bytes();
        let mut best: Option<u8> = None;
     
        for i in 0..num.len().saturating_sub(2) {
            if num[i] == num[i + 1] && num[i + 1] == num[i + 2] {
                if best.map_or(true, |b| num[i] > b) {
                    best = Some(num[i]);
                }
            }
        }

        return match best {
            Some(c) => std::iter::repeat(c as char).take(3).collect(),
            None => String::new()
        };
    }
}
