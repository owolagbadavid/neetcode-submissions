impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();

        let at_least = |k| {
            let mut count = HashMap::new();
            let mut res = 0;
            let mut has = 0;
            let mut l = 0;

            for r in 0..n {
                let entry = count.entry(nums[r]).or_insert(0);
                *entry += 1;
                if *entry == 1 {
                    has += 1;
                }

                while has >= k {
                    res += n - r;
                    let entry = count.entry(nums[l]).or_insert(0);
                    *entry -= 1;
                    if *entry == 0 {
                        has -= 1;
                    }
                    l += 1;
                }
            }

            res as i32
        };

        at_least(k) - at_least(k+1)
    }
}
