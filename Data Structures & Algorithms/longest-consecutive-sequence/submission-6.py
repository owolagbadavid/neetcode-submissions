class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        numSet = set(nums)
        res = 0
        for n in numSet:
            count = 1
            cur = n
            while n-1 not in numSet and cur+1 in numSet:
                count += 1
                cur += 1
            res = max(res, count)
        return res