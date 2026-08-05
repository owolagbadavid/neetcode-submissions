impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut i, mut j) = (m-1, n-1);

        let mut index = (m + n - 1) as usize;

        while i >= 0 || j >= 0 {
            if j < 0 {
                nums1[index] = nums1[i as usize];
                i -= 1;
                index -= 1;
            } else if i < 0 {
                nums1[index] = nums2[j as usize];
                j -= 1;
                index -= 1;
            } else if nums1[i as usize] > nums2[j as usize] {
                nums1[index] = nums1[i as usize];
                i -= 1;
                index -= 1;
            } else {
                nums1[index] = nums2[j as usize];
                j -= 1;
                index -= 1;
            }
        }
    }
}
