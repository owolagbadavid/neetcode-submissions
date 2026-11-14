impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort();
        let mut res = vec![0; spells.len()];

        for (i, spell) in spells.into_iter().enumerate() {
            let mut l = 0;
            let mut r = potions.len()-1;

            while l <= r {
                let m = (l + r) / 2;

                if (spell as i64 * potions[m] as i64) < success {
                    l = m + 1;
                } else if m == 0 {
                    break;
                } else {
                    r = m - 1;
                }
            }
            res[i] = (potions.len() - l) as i32;
        }

        res
    }
}
