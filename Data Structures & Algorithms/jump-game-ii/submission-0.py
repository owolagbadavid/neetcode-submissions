class Solution:
    def jump(self, nums: List[int]) -> int:
        n = len(nums)
        l = r = 0
        res = 0

        while r < n-1:
            m = l
            for i in range(l, r+1):
                m = max(m, i+nums[i])
            res += 1
            l = r+1
            r = m
        return res