class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        A, B = nums1, nums2
        total = len(nums1) + len(nums2)
        half = total // 2

        if len(A) > len(B):
            A, B = B, A

        l, r = 0, len(A) - 1
        while True:
            m = (l + r) // 2
            n = half - m - 2

            Al = A[m] if m >= 0 else float('-infinity')
            Ar = A[m+1] if (m + 1) < len(A) else float('infinity')
            Bl = B[n] if n >= 0 else float('-infinity')
            Br = B[n+1] if (n + 1) < len(B) else float('infinity')

            if Al <= Br and Bl <= Ar:
                return min(Ar, Br) if total % 2 else (max(Al, Bl) + min(Ar, Br)) / 2
            elif Al > Br:
                r = m - 1
            else:
                l = m + 1
