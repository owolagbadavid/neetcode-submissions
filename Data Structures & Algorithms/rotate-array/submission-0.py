class Solution:
    def rotate(self, nums: List[int], k: int) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        def reverse(lo, hi):
            while lo < hi:
                nums[lo], nums[hi] = nums[hi], nums[lo]
                lo += 1
                hi -= 1
        n = len(nums)
        k = k % n
        l, r, m = k-1, n-1, n-k-1
        reverse(0, r)
        reverse(0, l)
        reverse(k, r)



        