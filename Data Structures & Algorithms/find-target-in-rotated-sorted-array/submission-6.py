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
        
        def search(l: int, r: int):
            while l <= r:
                m = (l + r) // 2
                if target > nums[m]:
                    l = m + 1
                elif target < nums[m]:
                    r = m - 1
                else:
                    return m
            return -1

        l1, r1 = cut, len(nums) - 1
        l2, r2 = 0, cut - 1

        res1, res2 = search(l1, r1), search(l2, r2)
        return max(res1, res2) 
