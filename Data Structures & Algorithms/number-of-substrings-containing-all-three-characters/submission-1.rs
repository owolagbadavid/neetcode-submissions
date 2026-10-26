impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut count = [0; 3];
        let mut res = 0;
        let s = s.as_bytes();
        let n = s.len();

        let mut l = 0;
        for r in 0..s.len() {
            count[(s[r] - b'a') as usize] += 1;
            
            while count[0] > 0 && count[1] > 0 && count[2] > 0 {
                res += n - r;
                count[(s[l] - b'a') as usize] -= 1;
                l += 1;
            }
 
        }

        res as i32
    }
}
