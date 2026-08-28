impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a: Vec<u8> = a.bytes().rev().collect();
        let b: Vec<u8> = b.bytes().rev().collect();
        let (mut i, mut j) = (0, 0);
        let mut carry: Option<u8> = None;
        let mut res = String::new();

        while i < a.len() || j < b.len() || carry.is_some() {
            let mut cur = 0;
            if i < a.len() {
                cur += a[i] - 48;
                i += 1;
            }
            if j < b.len() {
                cur += b[j] - 48;
                j += 1;
            }
            if carry.is_some() {
                cur += carry.as_ref().unwrap();
                carry = None;
            }

            if cur > 1 {
                cur = if cur == 2 {0} else {1};
                carry = Some(1);
            }

            res.push((cur+48) as char);
        }

        res.chars().rev().collect()
    }
}
