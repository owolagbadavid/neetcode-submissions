class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        res = []
        nums.sort()

        for i in range(len(nums)):
            t = -nums[i]
            l, r = i+1, len(nums) - 1
            while l < r:
                cur = nums[r] + nums[l]
                if cur > t:
                    r -= 1
                elif cur < t:
                    l += 1
                else:
                    res.append((-t, nums[l], nums[r]))
                    l += 1
                    r -= 1
        return list(set(res))

