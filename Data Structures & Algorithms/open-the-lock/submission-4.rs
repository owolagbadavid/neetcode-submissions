impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let to_arr = |s: &str| -> [u8; 4] {
            let b = s.as_bytes();
            [b[0], b[1], b[2], b[3]]
        };

        let dead: HashSet<[u8; 4]> = deadends.iter().map(|s| to_arr(s)).collect();
        let start = *b"0000";
        let target = to_arr(&target);

        let mut visit = HashSet::from([start]);
        let mut q = VecDeque::from([start]);
        let mut res = 0;

        while !q.is_empty() {
            for _ in 0..q.len() {
                let cur = q.pop_front().unwrap();
                if dead.contains(&cur) {
                    continue;
                } else if cur == target {
                    return res;
                }
                for i in 0..4 {
                    for delta in [1u8, 9] {
                        let mut next = cur;
                        next[i] = b'0' + (next[i] - b'0' + delta) % 10;
                        if visit.insert(next) {
                            q.push_back(next);
                        }
                    }
                }
            }
            res += 1;
        }

        -1
    }
}