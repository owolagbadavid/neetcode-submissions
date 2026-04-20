class Solution:
    def isInterleave(self, s1: str, s2: str, s3: str) -> bool:
        n, m = len(s1), len(s2)
        if n + m != len(s3):
            return False
        dp = [[False]*(m+1) for _ in range(n+1)]
        dp[n][m] = True

        for j in range(m - 1, -1, -1):
            dp[n][j] = dp[n][j + 1] and s2[j] == s3[n + j]

        for i in range(n - 1, -1, -1):
            dp[i][m] = dp[i + 1][m] and s1[i] == s3[i + m]

        for i in range(n-1, -1, -1):
            for j in range(m-1, -1, -1):
                if s1[i] == s3[i+j]:
                    dp[i][j] = dp[i+1][j]
                if not dp[i][j] and s2[j] == s3[i+j]:
                    dp[i][j] = dp[i][j+1]
        
        return dp[0][0]
