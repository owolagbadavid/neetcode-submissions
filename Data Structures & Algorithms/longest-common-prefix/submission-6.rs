impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut res = String::new();
        let mut index = 0;
        loop {
            let mut cur = None;
            for i in 0..strs.len() {
                if index >= strs[i].len() {
                    return res;
                }
                let c = &strs[i][index..index+1];
                if cur.is_none() {
                    cur = Some(c);
                } else if cur.unwrap() != c {
                    return res;
                }

                if i == strs.len()-1 {
                    res += c;
                }

            }
            index += 1;
        }
        res
    }
}
