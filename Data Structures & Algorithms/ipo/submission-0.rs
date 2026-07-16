impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut res = w;
        let mut k = k;
        let mut heap = BinaryHeap::new();
        let mut min_heap = BinaryHeap::new();

        for i in 0..profits.len() {
            let p = profits[i];
            let c = capital[i];
            if c <= res {
                heap.push(p);
            } else {
                min_heap.push((-c, p));
            }
        }

        while !heap.is_empty() && k > 0 {
            let p = heap.pop().unwrap();
            res += p;
            k -= 1;
            while !min_heap.is_empty() && -min_heap.peek().unwrap().0 <= res {
                let (mut c, p) = min_heap.pop().unwrap();
                c = -c;
                heap.push(p);
            }
        }

        res
    }
}
