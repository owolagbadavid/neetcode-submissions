impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut count = [0; 26];
        let mut res = 0;

        for c in chars.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }

        let mut matcher = count;

        'outer: for word in words {
            matcher = count;
            for c in word.chars() {
                if c.is_ascii_lowercase() {
                    matcher[(c as u8 - b'a') as usize] -= 1;
                    if matcher[(c as u8 - b'a') as usize] < 0 {
                        continue 'outer;
                    }
                }
            }

            res += word.len() as i32
        }

        res
    }
}