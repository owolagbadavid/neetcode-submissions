class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        res = []
        def dfs(index, arr):
            if index >= len(nums):
                res.append([*arr])
                return
            
            arr.append(nums[index])
            dfs(index+1, arr)
            arr.pop()
            dfs(index+1, arr)
        dfs(0, [])
        return res