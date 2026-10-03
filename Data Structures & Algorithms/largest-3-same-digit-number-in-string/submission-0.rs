impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let num = num.as_bytes();
        let mut res: Option<&[u8]> = None;
        let mut max = 0;

        let mut set = HashSet::new();
        let mut l = 0;
        for r in 0..num.len() {
            while set.len() > 0 && !set.contains(&num[r]) {
                set.clear();
                l = r;
            }
            set.insert(num[r]);
            if num[l..=r].len() == 3 {
                let n: i32 = std::str::from_utf8(&num[l..=r]).unwrap().parse().unwrap();
                if n >= max {
                    max = n;
                    res = Some(&num[l..=r])
                }
            }
        }

        return String::from(
            match res {
                Some(slice) => std::str::from_utf8(slice).unwrap(),
                None => ""
            }
        );
    }
}
