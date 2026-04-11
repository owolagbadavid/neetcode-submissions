class Solution:
    def maxProduct(self, nums: List[int]) -> int:
        maxi = mini = nums[0]
        globalMax = nums[0]

        for i in range(1, len(nums)):
            newMax = maxi * nums[i]
            newMin = mini * nums[i]
            maxi = max(newMax, nums[i], newMin)
            mini = min(newMax, nums[i], newMin)
            globalMax = max(maxi, globalMax)

        return globalMax
