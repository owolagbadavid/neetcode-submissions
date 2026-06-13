class Solution:
    def minSubArrayLen(self, target: int, nums: List[int]) -> int:
        prefix = [0]
        for i in range(0, len(nums)):
            prefix.append(prefix[i]+nums[i])

        res = float('inf')
        for i in range(len(nums)):
            l, r = 0, i+1
            while l <= r:
                m = (l+r)//2
                if prefix[i+1] - prefix[m] >= target:
                    res = min(res, i-m+1)
                    l = m + 1
                else:
                    r = m - 1

        return res if res < float('inf') else 0