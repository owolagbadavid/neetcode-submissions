class Solution:
    def findMin(self, nums: List[int]) -> int:
        l, r = 0, len(nums) - 1

        while l < r:
            m = (l + r) // 2

            if nums[m] < nums[m-1]:
                return nums[m]
            
            if nums[l] < nums[m] and nums[l] < nums[r]:
                r -= 1
            else:
                l += 1
        return nums[l]