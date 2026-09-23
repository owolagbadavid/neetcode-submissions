impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut map = HashMap::new();

        for num in arr {
            *map.entry(num).or_insert(0) += 1;
        }

        let mut res = -1;

        for (k, v) in map {
            if k == v {
                res = res.max(k);
            }
        }

        res
    }
}
