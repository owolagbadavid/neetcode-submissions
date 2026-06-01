class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        memo = {}
        def dfs(i, have=None):
            if i >= len(prices):
                return 0
            
            if (i, have) in memo:
                return memo[(i, have)]
            
            if have is None:
                memo[(i, have)] = max(dfs(i+1, i), dfs(i+1, have))
                return memo[(i, have)]
            
            profit = prices[i]-prices[have]
            memo[(i, have)] = max(profit+dfs(i+1, None), dfs(i+1, have))
            return memo[(i, have)]
        return dfs(0)
            

                