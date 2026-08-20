

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut count = [(0, 0, "Radiant"), (0, 0, "Dire")];

        let mut q = VecDeque::new();

        for c in senate.chars() {
            let mut res = 1usize;
            if c == 'R' {
                count[0].0 += 1;
                res = 0;
            } else {
                count[1].0 += 1;
            }
            q.push_back(res);
        }

        while let Some(c) = q.pop_front() {
            let op = c ^ 1;
            if count[c].1 < 0 {
                count[c].1 += 1;
            } else if count[op].0 == 0 {
                return String::from(count[c].2);
            } else {
                count[op].0 -= 1;
                count[op].1 -= 1;
                q.push_back(c);
            }
        }

        String::new()
    }
}
