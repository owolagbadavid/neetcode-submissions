impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let (mut min, mut max) = (BinaryHeap::new(), BinaryHeap::new());
        let mut res = 0;

        let mut l = 0;
        for r in 0..nums.len() {
            min.push(Reverse((nums[r], r)));
            max.push((nums[r], r));

            while max.peek().unwrap().0 - min.peek().unwrap().0.0 > limit {
                while max.peek().unwrap().1 <= l {
                    max.pop();
                }
                while min.peek().unwrap().0.1 <= l {
                    min.pop();
                }
                l += 1;
            }

            res = res.max(r-l+1);
        }

        res as i32
    }
}
