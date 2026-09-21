impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut map = "balloon".chars().map(|c| (c, 0)).collect::<HashMap<char, i32>>();
        let mut res = 0;

        for c in text.chars() {
            *map.get_mut(&c).unwrap_or(&mut 1) += 1
        }

        'outer: loop {
            for c in "balloon".chars() {
                let count = map.get_mut(&c).unwrap();
                *count -= 1;
                if *count < 0 {
                    break 'outer;
                }
            }
            res += 1;
        }

        res
    }
}
