class Solution:
    def removeElement(self, nums: List[int], val: int) -> int:
        l, r = 0, len(nums)-1
        while r >= 0 and nums[r] == val:
            r -= 1
        while l <= r:
            if nums[l] == val:
                nums[l], nums[r] = nums[r], nums[l]
            while nums[r] == val:
                r -= 1
            l += 1
        return l