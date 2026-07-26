impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let order: HashMap<_, usize> = order.chars().enumerate().map(|(i, w)| (w, i)).collect();

        if words.len() < 2 {
            return true;
        }

        let mut l = 0;
        let mut r = 1;

        while r < words.len() {
            let r_len = words[r].len();
            let l_len = words[l].len();
            let len = min(r_len, l_len);

            if words[l][0..len] == words[r][0..len] && r_len < l_len {
                return false;
            }
            for i in 0..len {
                if order.get(&(words[r].as_bytes()[i] as char)).unwrap() > order.get(&(words[l].as_bytes()[i] as char)).unwrap() {
                    break;
                } else if order.get(&(words[r].as_bytes()[i] as char)).unwrap() < order.get(&(words[l].as_bytes()[i] as char)).unwrap() {
                    return false;
                }
            }
            r += 1;
            l += 1;
        }

        true
    }
}
