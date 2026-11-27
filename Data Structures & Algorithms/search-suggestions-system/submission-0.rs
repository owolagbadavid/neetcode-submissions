impl Solution {
    pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        products.sort();
        let mut res = vec![];
        let mut left = 0;
        let mut right = products.len() - 1;
        let mut prefix = String::new();

        for (i, &b) in search_word.as_bytes().into_iter().enumerate() {
            prefix.push(b as char);
            let (mut l, mut r) = (left, right);
            let left_word = products[left].as_bytes();
            if left_word.len() <= i || left_word[i] != b {
                while l < r {
                    let m = (r + l) / 2;
                    if products[m].as_str() >= prefix.as_str(){
                        r = m;
                    } else {
                        l = m + 1;
                    }
                }
            }
            left = l;

            let mut cur = vec![];

            for i in (left..=right).take(3) {
                if products[i].starts_with(&prefix) {
                    cur.push(products[i].clone());
                } else {
                    right = i;
                    break;
                }
            }

            res.push(cur);
        }

        res
    }
}
