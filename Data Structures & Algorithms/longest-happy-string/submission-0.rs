impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut heap = BinaryHeap::new();
        let mut res = String::new();

        if a > 0 { heap.push((a, 'a')) };
        if b > 0 { heap.push((b, 'b')) };
        if c > 0 { heap.push((c, 'c')) };

        let mut last = ('a', 0);

        while !heap.is_empty() {
            let mut cur = heap.pop().unwrap();
            let count = if last.0 == cur.1 { last.1 + 1 } else { 1 };
            if cur.1 == last.0 && last.1 == 2 {
                match heap.pop() { 
                    Some(mut next) => {
                        res += &next.1.to_string();
                        next.0 -= 1;
                        last = (next.1, 1);
                        if next.0 != 0 {
                            heap.push(next);
                        }
                    }
                    None => { return res; }
                }
            } else {
                res += &cur.1.to_string();
                cur.0 -= 1;
                last = (cur.1, count);
            }
            if cur.0 != 0 {
                heap.push(cur);
            }
        }
        res
    }
}
