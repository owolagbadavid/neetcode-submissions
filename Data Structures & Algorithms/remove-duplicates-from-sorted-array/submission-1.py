class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        l, r = -1, 0
        while r < len(nums):
            if l == -1 or nums[l] != nums[r]:
                l += 1
                nums[l] = nums[r]
            r += 1
        return l+1
