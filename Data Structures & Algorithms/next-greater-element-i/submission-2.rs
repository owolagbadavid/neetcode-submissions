impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut res = vec![-1; nums1.len()];
        let mut stack = vec![];

        for i in 0..nums2.len() {
            while stack.len() > 0 && *stack.last().unwrap() < nums2[i] {
                map.insert(stack.pop().unwrap(), nums2[i]);
            }
            stack.push(nums2[i]);
        }

        for i in 0..nums1.len() {
            res[i] = *map.get(&nums1[i]).unwrap_or(&-1);
        }

        res
    }
}
