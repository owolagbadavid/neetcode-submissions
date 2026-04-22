class Solution:
    def numDistinct(self, s: str, t: str) -> int:
        dp = {}
        def dfs(i, j):
            if j == len(t)-1:
                return 1
            if i == len(s)-1:
                return 0

            if (i,j) in dp:
                return dp[(i,j)]
        
            dp[(i,j)] = 0
            if i+1 < len(s) and j+1 < len(t) and s[i+1] == t[j+1]:
                dp[(i,j)] += dfs(i+1, j+1)
            
            dp[(i,j)] += dfs(i+1, j)
            return dp[(i,j)]
        return dfs(-1, -1)