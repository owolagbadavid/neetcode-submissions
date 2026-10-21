impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut res = 0;
        let k = k as usize;
        let mut l = 0;
        let mut vowel_count = 0;
        let vowels = HashSet::from(*b"aeiou");

        for r in 0..s.len() {
            if vowels.contains(&s[r]) {
                vowel_count += 1;
            }

            if r - l + 1 == k {
                res = res.max(vowel_count);
                if vowels.contains(&s[l]) {
                    vowel_count -= 1;
                }
                l += 1;
            }
        }

        res
    }
}
