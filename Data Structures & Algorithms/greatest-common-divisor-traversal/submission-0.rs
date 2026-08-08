impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        let mut visit = vec![false; nums.len()];
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if Self::gcd(nums[i], nums[j]) > 1 {
                    visit[i] = true;
                    visit[j] = true;
                }
            }
        }
        visit.into_iter().all(|e| e)
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 {
            b
        } else if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}
