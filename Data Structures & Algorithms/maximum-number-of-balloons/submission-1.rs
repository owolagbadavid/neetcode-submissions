impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut map = "balloon".chars().map(|c| (c, 0)).collect::<HashMap<char, i32>>();
        let mut res = 10000;

        for c in text.chars() {
            *map.get_mut(&c).unwrap_or(&mut 1) += 1
        }

        for c in "balon".chars() {
            let mut count = *map.get_mut(&c).unwrap();
            if c == 'l' || c == 'o' {
                count /= 2;
            }
            res = res.min(count);
        }

        res
    }
}
