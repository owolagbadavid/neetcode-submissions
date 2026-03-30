class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        longest = 0
        numSet = set(nums)
        for n in nums:
            cur = 1
            if n-1 not in numSet:
                n2 = n
                while n2+1 in numSet:
                    cur += 1
                    n2 += 1
                longest = max(longest, cur)
        return longest