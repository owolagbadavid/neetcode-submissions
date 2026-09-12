impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut map = HashMap::new();

        let mut o_max = i32::MIN;
        let mut e_min = i32::MAX;

        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        for v in map.values() {
            if *v % 2 == 0 {
                e_min = e_min.min(*v);
            } else {
                o_max = o_max.max(*v);
            }
        }

        o_max - e_min
    }
}
