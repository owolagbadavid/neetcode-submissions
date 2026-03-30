class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        arr = []
        l = r = 0
        while l < len(nums1) or r < len(nums2):
            if l >= len(nums1):
                arr.append(nums2[r])
                r += 1
            elif r >= len(nums2):
                arr.append(nums1[l])
                l += 1
            elif nums1[l] < nums2[r]:
                arr.append(nums1[l])
                l += 1
            else:
                arr.append(nums2[r])
                r += 1
        
        l, r = 0, len(arr) - 1
        m = (l + r) // 2
        return arr[m] if len(arr) % 2 else (arr[m] + arr[m+1]) / 2
