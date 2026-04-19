class Solution:
    def findTargetSumWays(self, nums: List[int], target: int) -> int:
        dp = {}

        def dfs(i, left):
            if i == len(nums) and left == 0:
                return 1
            if i >= len(nums):
                return 0
            
            if (i,left) in dp:
                return dp[(i,left)]
            dp[(i,left)] = dfs(i+1, left-nums[i]) + dfs(i+1, left+nums[i])
            return dp[(i,left)]
        return dfs(0, target)