class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        n = len(nums)
        total = sum(nums)
        target = total // 2
        if total % 2:
            return False

        dp = [[False]*(target+1) for _ in range(n+1)]
        dp[0][0] = True
        for i in range(1, n+1):
            for j in range(target+1):
                dp[i][j] = dp[i-1][j]
                if j - nums[i-1] >= 0:
                    dp[i][j] = dp[i][j] or dp[i-1][j - nums[i-1]]
        
        return dp[n][target]
