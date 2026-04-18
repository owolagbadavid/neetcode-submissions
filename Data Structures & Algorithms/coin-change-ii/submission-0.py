class Solution:
    def change(self, amount: int, coins: List[int]) -> int:
        dp = {}
        def dfs(i, left):
            if left == 0:
                return 1
            if i >= len(coins):
                return 0
            if (i,left) in dp:
                return dp[(i,left)]

            dp[(i,left)] = dfs(i+1, left)
           
            if left - coins[i] >= 0:
                dp[(i,left)] += dfs(i, left-coins[i])
            return dp[(i,left)]
        
        return dfs(0, amount)
