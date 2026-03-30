class Solution:
    def search(self, nums: List[int], target: int) -> int:
        l, r = 0, len(nums) - 1
        cut = 0

        while l <= r:
            m = (l + r) // 2
            if nums[m] < nums[m-1]:
                cut = m
                break
            if nums[m] >= nums[l] and nums[r] <= nums[l]:
                l = m + 1
            else:
                r = m - 1
        
        l1, r1 = cut, len(nums) - 1
        l2, r2 = 0, cut - 1

        while l1 <= r1:
            m = (l1 + r1) // 2
            if target > nums[m]:
                l1 = m + 1
            elif target < nums[m]:
                r1 = m - 1
            else:
                return m
        
        while l2 <= r2:
            m = (l2 + r2) // 2
            if target > nums[m]:
                l2 = m + 1
            elif target < nums[m]:
                r2 = m - 1
            else:
                return m

        return -1