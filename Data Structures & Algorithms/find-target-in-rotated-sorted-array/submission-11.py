class Solution:
    def search(self, nums: List[int], target: int) -> int:
        l, r = 0, len(nums) - 1

        while l < r:
            m = (l + r) // 2

            if nums[m] < nums[r]:
                r = m
            else:
                l = m + 1
        
        l1, r1 = l, len(nums) - 1
        l2, r2 = 0, l-1

        print(nums[l])
        while l1 <= r1:
            m = (l1 + r1) // 2

            if nums[m] == target:
                return m
            elif nums[m] > target:
                r1 = m - 1
            else:
                l1 = m + 1
            
        while l2 <= r2:
            m = (l2 + r2) // 2

            if nums[m] == target:
                return m
            elif nums[m] > target:
                r2 = m - 1
            else:
                l2 = m + 1
        
            
        return -1
