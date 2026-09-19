impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut map = (1..=nums.len()).map(|i| (i as i32, false)).collect::<HashMap<i32, bool>>();

        for num in &nums {
            *map.get_mut(num).unwrap() = true;
        }

        let mut res = vec![];
        for (k, v) in map {
            if !v {
                res.push(k);
            }
        }

        res 
    }
}
