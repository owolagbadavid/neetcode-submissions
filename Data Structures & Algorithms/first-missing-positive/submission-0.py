class Solution:
    def firstMissingPositive(self, nums: List[int]) -> int:
        nums = set(nums)
        cur = 1
        while True:
            if cur not in nums:
                return cur
            cur += 1