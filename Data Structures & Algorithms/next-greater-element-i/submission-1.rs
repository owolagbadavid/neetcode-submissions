impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut res = vec![-1; nums1.len()];

        for i in 0..nums2.len() {
            map.insert(nums2[i], i);
        }

        for i in 0..nums1.len() {
            for j in map[&nums1[i]]..nums2.len() {
                if nums2[j] > nums1[i] {
                    res[i] = nums2[j];
                    break;
                }
            }
        }

        res
    }
}
