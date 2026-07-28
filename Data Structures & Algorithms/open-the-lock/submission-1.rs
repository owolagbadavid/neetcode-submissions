impl Solution {
    pub fn open_lock(mut deadends: Vec<String>, target: String) -> i32 {
        let mut res = 0;
        let mut visit = HashSet::from([String::from("0000")]);
        let deadends: HashSet<String> = deadends.into_iter().collect();

        let mut q = VecDeque::from([String::from("0000")]);

        if deadends.contains("0000") {
            return -1;
        }

        while !q.is_empty() {
            for _ in 0..q.len() {
                let cur = q.pop_front().unwrap();
                if cur == target {
                    return res;
                }
                for i in 0..4 {
                    let val: i32 = cur[i..i+1].parse().unwrap();
                    let mut up = cur.clone();
                    up.replace_range(i..i+1, &((val + 1) % 10).to_string());
                    if !deadends.contains(&up) && !visit.contains(&up) {
                        q.push_back(up.to_owned());
                        visit.insert(up.to_owned());
                    }
                    let mut down = cur.clone();
                    down.replace_range(i..i+1, &((val - 1 + 10) % 10).to_string());
                    if !deadends.contains(&down) && !visit.contains(&down) {
                        q.push_back(down.to_owned());
                        visit.insert(down.to_owned());
                    }
                }
            }
            res += 1;
        }

        -1
    }
}
