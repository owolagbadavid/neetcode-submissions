impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut meeting_count: HashMap<i32, i32> = (0..n).map(|m| (m, 0)).collect();
        let mut in_heap = BinaryHeap::new();
        let mut out_heap: BinaryHeap<Reverse<[i32; 2]>> = BinaryHeap::new();
        let mut room_heap: BinaryHeap<Reverse<i32>> = (0..n).map(|r| Reverse(r)).collect();

        for m in &meetings {
            in_heap.push(Reverse([m[0], m[1]]));
        }

        while let Some(meet) = in_heap.pop() {
            let meet = meet.0;

            while let Some(last) = out_heap.peek() && last.0[0] <= meet[0] {
                let last = out_heap.pop().unwrap();
                let last = last.0;
                room_heap.push(Reverse(last[1]));
            }

            if let Some(next) = room_heap.pop() {
                let next = next.0;
                out_heap.push(Reverse([meet[1], next]));
                *meeting_count.get_mut(&next).unwrap() += 1;
            } else {
                let last = out_heap.pop().unwrap();
                let last = last.0;
                let delay = last[0] - meet[0];
                *meeting_count.get_mut(&last[1]).unwrap() += 1;
                out_heap.push(Reverse([meet[1]+ delay, last[1]]));
            }
        }

        meeting_count.into_iter().min_by_key(|(k, v)| (-(*v), *k)).unwrap().0
    }
}
