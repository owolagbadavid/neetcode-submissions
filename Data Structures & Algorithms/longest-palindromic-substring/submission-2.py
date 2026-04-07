class Solution:
    def longestPalindrome(self, s: str) -> str:
        res = s[0]
        resLen = 1
        
        for i in range(len(s)):
            l = r = i
            while l-1 >= 0 and r + 1 < len(s) and s[l-1] == s[r+1]:
                l -= 1
                r += 1
                if r-l+1 > resLen:
                    res = s[l:r+1]
                    resLen = r-l+1
            l = r = i
            while l >= 0 and r + 1 < len(s) and s[l] == s[r+1]:
                l -= 1
                r += 1
                if r-l > resLen:
                    res = s[l+1:r+1]
                    resLen = r-l
            
        return res



                