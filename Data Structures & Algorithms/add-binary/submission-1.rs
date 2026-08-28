impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a: Vec<u8> = a.bytes().rev().collect();
        let b: Vec<u8> = b.bytes().rev().collect();
        let (mut i, mut j, mut carry) = (0, 0, 0);
        let mut res = String::new();

        while i < a.len() || j < b.len() || carry > 0 {
            let mut cur = 0;
            if i < a.len() {
                cur += a[i] - 48;
                i += 1;
            }

            if j < b.len() {
                cur += b[j] - 48;
                j += 1;
            }

            cur += carry;

            carry = cur >> 1;
            cur = cur & 1;

            res.push((cur+48) as char);
        }

        res.chars().rev().collect()
    }
}
