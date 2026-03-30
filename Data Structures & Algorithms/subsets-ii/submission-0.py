class Solution:
    def subsetsWithDup(self, nums: List[int]) -> List[List[int]]:
        res = []
        nums.sort()

        def back(index, arr):
            if index >= len(nums):
                res.append([*arr])
                return
            
            arr.append(nums[index])
            back(index + 1, arr)
            arr.pop()

            while index + 1 < len(nums) and nums[index + 1] == nums[index]:
                index += 1
            
            back(index + 1, arr)
        
        back(0, [])
        return res