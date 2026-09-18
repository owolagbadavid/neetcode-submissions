#[derive(Debug)]
struct NumArray {
    prefix: Vec<i32>
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut prefix = vec![0; nums.len()];
        for i in 0..nums.len() {
            prefix[i] = if i == 0 { 0 } else { prefix[i-1] }  + nums[i];
        }
        Self { prefix }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefix[right as usize] - if left == 0 { 0 } else { self.prefix[(left - 1) as usize] }
    }
}
