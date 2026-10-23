impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n = code.len();
        let ni = n as i32;
        let mut o = k;
        let mut res = vec![0; n];

        let index = |i: i32| i.rem_euclid(ni) as usize;

        let mut sum = 0;

        if k < 0 {
            for i in (k..0).rev() {
                sum += code[index(i)];
            }
        } else {
            for i in 1..=k {
                sum += code[index(i)];
            }
        }

        res[0] = sum;

        for i in 1..n {
            if k < 0 {
                sum -= code[index(o)];
                sum += code[index((i-1) as i32)];
            } else {
                sum -= code[index(i as i32)];
                sum += code[index(o+1)];
            }
            o += 1;
            res[i] = sum;
        }

        res
    }
}
