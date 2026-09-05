impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        let mut res = 0;
        for s in details {
            let num: i32 = s[11..=12].parse().unwrap();
            if num > 60 {
                res += 1;
            }
        }

        res
    }
}
