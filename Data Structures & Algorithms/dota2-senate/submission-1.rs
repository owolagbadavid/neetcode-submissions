impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        const NAMES: [&str; 2] = ["Radiant", "Dire"];

        let mut alive = [0i32; 2];
        let mut pending = [0i32; 2];
        let mut q = VecDeque::with_capacity(senate.len());

        for b in senate.bytes() {
            let i = usize::from(b == b'D');
            alive[i] += 1;
            q.push_back(i);
        }

        while let Some(i) = q.pop_front() {
            let op = i ^ 1;
            if pending[i] > 0 {
                pending[i] -= 1;
            } else if alive[op] == 0 {
                return NAMES[i].to_string();
            } else {
                alive[op] -= 1;
                pending[op] += 1;
                q.push_back(i);
            }
        }
        unreachable!("one side always eliminates the other")
    }
}