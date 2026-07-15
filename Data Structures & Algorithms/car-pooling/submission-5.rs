impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut heap = BinaryHeap::new();
        let mut heap_end: BinaryHeap<Reverse<(i32, i32, i32)>> = BinaryHeap::new();
        for i in 0..trips.len() {
            let trip = &trips[i];
            heap.push(Reverse((trip[1], trip[2], trip[0])));
        }

        let mut cap_left = capacity;
        let mut last_interval_end = 0;
        while !heap.is_empty() {
            let trip = heap.pop().unwrap().0;

            while !heap_end.is_empty() {
                if trip.0 >= heap_end.peek().unwrap().0.0 {
                    cap_left += heap_end.pop().unwrap().0.2
                } else {
                    break;
                }
            }

            cap_left -= trip.2;
            if cap_left < 0 {
                return false;
            }

            heap_end.push(Reverse((trip.1, trip.0, trip.2)));
        }

        true
    }
}
