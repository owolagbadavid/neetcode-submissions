impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sofar = vec![];
        let mut res = vec![];

        Self::backtrack(0, n, &mut sofar, &mut res, &nums);

        res.iter().fold(0, |acc, e| acc + e) as i32
    }

    fn backtrack(i: usize, n: usize, sofar: &mut Vec<usize>, res: &mut Vec<usize>, nums: &Vec<i32>) {
        if i >= n {
            return res.push(sofar.iter().fold(0 as usize, |acc, e| acc ^ e));
        }

        Self::backtrack(i+1, n, sofar, res, nums);
        sofar.push(nums[i] as usize);

        Self::backtrack(i+1, n, sofar, res, nums);
        sofar.pop();
    } 
}
