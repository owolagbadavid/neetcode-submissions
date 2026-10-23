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
            let i = i as i32;
            if k < 0 {
                sum -= code[index(k + i - 1)];
                sum += code[index(i-1)];
            } else {
                sum -= code[index(i)];
                sum += code[index(k+i)];
            }
            o += 1;
            res[i as usize] = sum;
        }

        res
    }
}
