class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        numSet = set(nums)
        res = 0
        for i in range(0, len(nums)):
            if nums[i] - 1 not in nums:
                cur = 1
                n = nums[i]
                while n + 1 in numSet:
                    cur += 1
                    n += 1
                res = max(cur, res)
        return res