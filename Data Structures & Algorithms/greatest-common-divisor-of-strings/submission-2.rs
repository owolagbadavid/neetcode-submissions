impl Solution {
    pub fn gcd_of_strings(mut str1: String, mut str2: String) -> String {
        let mut res = String::new();

        if str1.len() > str2.len() {
            (str1, str2) = (str2, str1);
        }

        let (n1, n2) = (str1.len(), str2.len());
        let mut start = 0; 
        for i in 0..str1.len() {
            let cand = res.clone() + &str1[start..=i];
            let n = cand.len();
            if n1 % n == 0 && n2 % n == 0 && cand.repeat(n1/n) == str1 && cand.repeat(n2/n) == str2 {
                res = cand;
                start = i + 1;
            }
        }

        res
    }
}
