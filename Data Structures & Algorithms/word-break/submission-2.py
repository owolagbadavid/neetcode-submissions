class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        dp = {}

        def dfs(i):
            if i >= len(s):
                return True

            if i in dp:
                return dp[i]

            dp[i] = False
            for word in wordDict:
                n = len(word)
                if s[i:i+n] == word:
                    dp[i] = dfs(i+n)
                    if dp[i]:
                        return dp[i]

            return dp[i]

        return dfs(0)
