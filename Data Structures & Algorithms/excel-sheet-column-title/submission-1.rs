impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut chars = vec![];

        while column_number >= 1 {
            column_number -= 1;
            chars.push(column_number % 26);
            column_number /= 26;
        }

        let res: String = chars.into_iter().rev().map(|c| char::from((c + 65) as u8)).collect();

        res
    }
}
