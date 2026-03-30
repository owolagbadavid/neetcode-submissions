class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        longest = 1 if len(nums) >= 1 else 0
        numSet = set(nums)
        for n in nums:
            cur = 1
            if n+1 in numSet and n-1 not in numSet:
                n2 = n
                while n2+1 in numSet:
                    cur += 1
                    longest = max(longest, cur)
                    n2 += 1
        return longest