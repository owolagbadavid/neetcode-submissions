impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let num = nums[i].abs() as usize;
            nums[num-1] = nums[num-1].abs() * -1;
        }

        let mut res = vec![];
        for i in 0..nums.len() {
            if nums[i] > 0 {
                res.push((i+1) as i32)
            }
        }

        res 
    }
}
