impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut perm = vec![];
        let mut map = HashMap::new();

        for i in 0..nums.len() {
            let count = map.entry(nums[i]).or_insert(0);
            *count += 1
        }

        let keys: Vec<i32> = map.keys().copied().collect();

        Self::backtrack(nums.len(), &keys, &mut map, &mut res, &mut perm);
        res
    }

    fn backtrack(n: usize, keys: &Vec<i32>, nums: &mut HashMap<i32, i32>, res: &mut Vec<Vec<i32>>, perm: &mut Vec<i32>) {
        if perm.len() == n {
            res.push(perm.clone());
            return;
        }

        for k in keys {
            let v = nums.get_mut(k).unwrap();
            if *v > 0 {
                perm.push(*k);
                *v -= 1;
                Self::backtrack(n, keys, nums, res, perm);
                *nums.get_mut(&k).unwrap() += 1;
                perm.pop();
            }
        }
    }
}
