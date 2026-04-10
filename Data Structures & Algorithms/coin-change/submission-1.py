class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        n = len(coins)
        dp = [0]*(amount+1)

        for i in range(1, amount+1):
            res = float('inf')
            for coin in coins:
                if i - coin >= 0:
                    res = min(res, 1 + dp[i-coin])
            dp[i] = res
        
        return dp[amount] if dp[amount] < float('inf') else -1
