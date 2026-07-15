impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut heap = BinaryHeap::new();
        for i in 0..trips.len() {
            let trip = &trips[i];
            heap.push(Reverse((trip[1], trip[2], trip[0])));
        }

        let mut cap_left = capacity;
        let mut last_interval_end = 0;
        while !heap.is_empty() {
            let trip = heap.pop().unwrap().0;
            // starts after last trip
            if trip.0 >= last_interval_end {
                cap_left = capacity;
            }

            cap_left -= trip.2;
            if cap_left < 0 {
                return false;
            }

            if let Some(next) = heap.peek() {
                // next trip starts after last trip
                let next = next.0;
                if next.0 >= last_interval_end {
                    cap_left = capacity - trip.2;
                }
            }

            last_interval_end = max(last_interval_end, trip.1);
        }

        true
    }
}
