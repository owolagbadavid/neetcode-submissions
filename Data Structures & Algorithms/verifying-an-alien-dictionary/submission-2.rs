impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let order: HashMap<u8, usize> = order.bytes().enumerate().map(|(i, c)| (c, i)).collect();

        words.windows(2).all(|w| {
            let (a, b) = (w[0].as_bytes(), w[1].as_bytes());
            let len = min(a.len(), b.len());
            for i in 0..len {
                match order[&a[i]].cmp(&order[&b[i]]) {
                    Ordering::Less => return true,
                    Ordering::Greater => return false,
                    Ordering::Equal => continue,
                }
            }
            a.len() <= b.len()
        })
    }
}
