impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut l = vec![1; n];
        let mut r = vec![1; n];
        let mut res = 0;

        for i in 1..n {
            if ratings[i] > ratings[i-1] {
                l[i] += l[i-1];
            }
        }

        for i in (0..n-1).rev() {
            if ratings[i] > ratings[i+1] {
                r[i] += r[i+1];
            }
        }

        for i in 0..n {
            res += l[i].max(r[i]);
        }

        res
    }
}
