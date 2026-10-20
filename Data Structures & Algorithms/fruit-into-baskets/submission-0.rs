impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut res = 0;

        let mut l = 0;

        for r in 0..fruits.len() {
            *count.entry(fruits[r]).or_insert(0) += 1;
            while count.keys().len() > 2 {
                let key = fruits[l];
                let value = count.get_mut(&key).unwrap();
                l += 1;
                *value -= 1;
                if *value == 0 {
                    count.remove(&key);
                }
            }

            res = res.max(r-l+1)
        }

        res as i32
    }
}
