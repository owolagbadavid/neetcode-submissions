impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let allowed: HashSet<char> = allowed.chars().collect();   
        let mut res = 0;

        'outer: for word in words {
            for c in word.chars() {
                match allowed.get(&c) {
                    Some(_) => {}
                    None => continue 'outer
                }
            }
            res += 1
        }

        res
    }
}