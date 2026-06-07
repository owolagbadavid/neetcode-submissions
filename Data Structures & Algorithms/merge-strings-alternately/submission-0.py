class Solution:
    def mergeAlternately(self, word1: str, word2: str) -> str:
        i = j = 0
        n, m = len(word1), len(word2)
        res = ""
        while i < n or j < m:
            if i >= n:
                res += word2[i:]
                j = m
            elif j >= m:
                res += word1[j:]
                i = n
            else:
                res += word1[i] + word2[j]
                i += 1
                j += 1
        return res