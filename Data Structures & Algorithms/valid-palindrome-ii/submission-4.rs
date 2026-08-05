impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let is_valid = |mut l: usize, mut r: usize| {
            while l < r {
                if s[l] != s[r] {
                    return false;
                }
                l += 1;
                r -= 1;
            }
            true
        };

        let (mut l, mut r) = (0, s.len()-1);

        while l < r {
            if s[l] == s[r] {
                l += 1;
                r -= 1;
            } else {
                return is_valid(l, r-1) || is_valid(l+1, r);
            }
        }

        true
    }
}
