impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map = HashMap::new();
        let mut set = HashSet::new();
        
        let s = s.as_bytes();
        let t = t.as_bytes();

        for i in 0..s.len() {
            if !map.contains_key(&s[i]) && !set.contains(&t[i]) {
                map.insert(s[i], t[i]);
                set.insert(t[i]);
            }

            if *map.get(&s[i]).unwrap_or(&0) != t[i] {
                return false;
            }
        }

        true
    }
}
