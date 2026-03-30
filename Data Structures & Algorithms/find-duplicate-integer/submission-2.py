class Solution:
    def findDuplicate(self, nums: List[int]) -> int:
        res = 0
        for i in range(len(nums)):
            index = abs(nums[i]) - 1
            if nums[index] < 0:
                return index + 1
            nums[index] *= -1
        return 0