class Solution:
    def findMin(self, nums: List[int]) -> int:
        l, r = 0, len(nums) - 1
        res = nums[0]
        while l <= r:
            m = (l+r) // 2
            if nums[m] < nums[m - 1]:
                return nums[m]
            elif nums[l] <= nums[m] and nums[r] <= nums[m]:
                l = m + 1
            else:
                r = m - 1
        return res