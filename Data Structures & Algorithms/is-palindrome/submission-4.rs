impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<char> = s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();
        let (mut l, mut r) = (0, s.len()-1);

        if s.len() > 0 {
            while l < r {
                if s[l] != s[r] {
                    return false
                }
                l += 1;
                r -= 1;
            }
        }

        true
    }
}
