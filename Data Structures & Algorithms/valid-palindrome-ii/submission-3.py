class Solution:
    def validPalindrome(self, s: str) -> bool:
        l, r = 0, len(s)-1
        def isValid(i, j):
            while i < j:
                if s[i] != s[j]:
                    return False
                i += 1
                j -= 1
            return True

        while l < r:
            if s[l] == s[r]:
                l += 1
                r -= 1
                continue

            return isValid(l, r-1) or isValid(l+1, r)

        return True