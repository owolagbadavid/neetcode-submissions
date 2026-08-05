impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let mut s = String::new();

        let (mut i, mut j) = (0, 0);
        while i < word1.len() || j < word2.len() {
            if i >= word1.len() {
                s.push(word2[j] as char);
                j += 1;
            } else if j >= word2.len() {
                s.push(word1[i] as char);
                i += 1;
            } else {
                s.push(word1[i] as char);
                i += 1;
                s.push(word2[j] as char);
                j += 1;
            }
        }

        s
    }
}
