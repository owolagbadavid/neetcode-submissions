impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut res = vec![];

        for i in 0..words.len() {
            for j in 0..words.len() {
                if i == j || !words[j].contains(&words[i]) {
                    continue;
                }
                res.push(words[i].clone());
                break;
            }
        }

        res
    }
}
