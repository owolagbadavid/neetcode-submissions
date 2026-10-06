impl Solution {
    pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
        let (mut i, mut j) = (0, 0);
        let word = word.as_bytes();
        let abbr = abbr.as_bytes();
        let mut n: Vec<u8> = vec![];

        while i < abbr.len() && j < word.len() {
            let c = abbr[i];
            if c.is_ascii_digit() {
                if n.len() == 1 && n[0] == b'0' {
                    return false;
                }
                n.push(c);
            } else {
                let num: usize = n.iter().fold(0, |acc, &b| acc * 10 + (b - b'0') as usize);
                j += num;
                if j < word.len() && word[j] != abbr[i] {
                    return false
                }

                n.clear();
                j += 1;
            }
            i += 1;
        }

        let num: usize = n.iter().fold(0, |acc, &b| acc * 10 + (b - b'0') as usize);
        j += num;
        if j == word.len() && i == abbr.len() {
            return true;
        }

        false
    }
}
