class Solution:
    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        """
        Do not return anything, modify nums1 in-place instead.
        """
        copy = nums1.copy()
        i = j = 0
        index = 0
        while i < m or j < n:
            if i >= m:
                nums1[index] = nums2[j]
                j += 1
            elif j >= n:
                nums1[index] = copy[i]
                i += 1
            elif copy[i] < nums2[j]:
                nums1[index] = copy[i]
                i += 1
            else:
                nums1[index] = nums2[j]
                j += 1
            index += 1
