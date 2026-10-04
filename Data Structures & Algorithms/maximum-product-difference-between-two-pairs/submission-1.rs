impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut min_heap = BinaryHeap::new();
        let mut max_heap = BinaryHeap::new();

        for n in nums {
            max_heap.push(n);
            min_heap.push(Reverse(n));
        }

        (max_heap.pop().unwrap() * max_heap.pop().unwrap()) - (min_heap.pop().unwrap().0 * min_heap.pop().unwrap().0)
    }
}