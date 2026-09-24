class Solution:
    def isArraySpecial(self, nums: List[int]) -> bool:
        i, j = 0, 1
        while j < len(nums):
            if nums[i] % 2 == nums[j] % 2:
                return False
            j += 1
            i += 1

        return True