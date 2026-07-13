impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let len = s.len();
        let mut counter = HashMap::new();
        let mut max_num = 0;

        for i in 0..len {
            let mut count = counter.entry(String::from(&s[i..i+1])).or_insert(0);
            *count += 1;
            max_num = max(*count, max_num);
        }

        if 2 * max_num > len + 1 {
            return String::from("");
        }

        let mut heap = BinaryHeap::new();
        let mut res = String::new();

        for (k, v) in &counter {
            heap.push((*v, k.to_owned()));
        }

        let mut last = String::from("A");
        while !heap.is_empty() {
            let mut cur = heap.pop().unwrap();
            if cur.1 != last {
                res += &cur.1;
                last = cur.1.to_owned();
                cur.0 -= 1;
                if cur.0 != 0 {
                    heap.push(cur);
                }
            } else {
                if let Some(mut next) = heap.pop() {
                    res += &next.1;
                    last = next.1.to_owned();
                    next.0 -= 1;
                    if next.0 != 0 {
                        heap.push(next);
                    }
                    heap.push(cur)
                } else {
                    return String::from("");
                }
            }
        }
 
        res
    }
}
