class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        n = len(nums)
        total = sum(nums)
        target = total // 2
        if total % 2:
            return False

        dp = {}

        def dfs(i,t):
            if i >= len(nums):
                return False
            if t == 0:
                return True

            if (i,t) in dp:
                return dp[(i,t)]

            dp[(i,t)] = dfs(i+1,t)

            if t - nums[i] >= 0:
                dp[(i,t)] = dp[(i,t)] or dfs(i+1, t-nums[i])

            return dp[(i,t)]
        return dfs(0,target)