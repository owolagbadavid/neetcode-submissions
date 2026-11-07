impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        let word = word.as_bytes();
        let n = word.len();

        let at_least = |k| {
            let mut res = 0;

            let mut count: HashMap<u8, i32> = "aeiou".bytes().map(|c| (c, 0)).collect();
            let mut cons_count = 0;
            let mut has = 0;

            let mut l = 0;

            for r in 0..word.len() {
                if let Some(entry) = count.get_mut(&word[r]) {
                    *entry += 1;
                    has += if *entry == 1 {1} else {0};
                } else {
                    cons_count += 1;
                }

                while has == 5 && cons_count >= k {
                    res += n - r;
                    if let Some(entry) = count.get_mut(&word[l]) {
                        *entry -= 1;
                        has -= if *entry == 0 {1} else {0};
                    } else {
                        cons_count -= 1;
                    }
                    l += 1;
                }
            }

            res as i64
        };

        at_least(k) - at_least(k+1) 
    }
}
