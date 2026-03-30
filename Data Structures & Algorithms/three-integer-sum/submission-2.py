class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        res = []

        sort = sorted(nums)

        for i in range(len(nums)):
            t = -sort[i]
            l, r = i+1, len(nums) - 1
            while l < r:
                cur = sort[r] + sort[l]
                if cur > t:
                    r -= 1
                elif cur < t:
                    l += 1
                else:
                    res.append((-t, sort[l], sort[r]))
                    l += 1
                    r -= 1
        return list(set(res))

