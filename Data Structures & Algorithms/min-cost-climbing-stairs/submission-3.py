class Solution:
    def minCostClimbingStairs(self, cost: List[int]) -> int:
        n = len(cost)
        dp = [0, 0]

        for i in range(n-1, -1, -1):
            tmp = dp[1]
            dp[1] = cost[i] + min(dp)
            dp[0] = tmp
        
        return min(dp)