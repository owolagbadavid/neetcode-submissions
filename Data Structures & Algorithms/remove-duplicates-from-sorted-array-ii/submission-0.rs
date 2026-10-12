impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut count = HashMap::new();

        let mut l = 0;
        for r in 0..nums.len() {
            let entry = count.entry(nums[r]).or_insert(0);
            if *entry < 2 {
                nums[l] = nums[r];
                *entry += 1;
                l += 1;
            }
        }

        l as i32
    }
}