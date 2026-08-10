impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let mut dp = HashMap::from([(1,1), (2, 1)]);

        for num in 2..=n {
            dp.insert(num, if num == n {0} else {num});
            for k in 1..num {
                let cur = *dp.get_mut(&num).unwrap();
                let l = *dp.get(&k).unwrap();
                let r = *dp.get(&(num-k)).unwrap();
                *dp.get_mut(&num).unwrap() = max(cur, l * r)
            }
        }

        *dp.get(&n).unwrap()
    }
}
