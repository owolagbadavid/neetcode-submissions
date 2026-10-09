impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = Vec::with_capacity(n);

        let mut i = nums.iter().position(|&x| x >= 0).unwrap_or(n) as i32;
        let mut j = i - 1;

        while i < n as i32 {
            let jnum = if j >= 0 { nums[j as usize].abs() } else { i32::MAX };
            let inum = nums[i as usize];
            if jnum < inum {
                res.push(jnum.pow(2));
                j -= 1;
            } else {
                res.push(inum.pow(2));
                i += 1;
            }
        }

        while j >= 0 {
            res.push(nums[j as usize].pow(2));
            j -= 1;
        }

        res
    }
}
