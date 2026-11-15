use std::cmp::Reverse;

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut heap = BinaryHeap::new();
        let mut max = i32::MIN;

        for i in 0..nums.len() {
            let num = nums[i][0];
            heap.push(Reverse((num, 0, i)));
            max = max.max(num);
        }

        let mut range = (heap.peek().unwrap().0.0 - max).abs();
        let mut res = vec![heap.peek().unwrap().0.0, max];
        loop {
            let (num, j, i) = heap.pop().unwrap().0;
            if j >= nums[i].len() - 1 {
                break;
            }
            let next = nums[i][j+1];
            max = max.max(next);
            heap.push(Reverse((next, j+1, i)));

            let new_min = heap.peek().unwrap().0.0;
            let cand = (max - new_min).abs();
            if range > cand {
                range = cand;
                res[0] = new_min;
                res[1] = max;
            }
        }

        res
    }
}
