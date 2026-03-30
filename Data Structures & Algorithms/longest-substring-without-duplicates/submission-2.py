class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        if len(s) <= 1:
            return len(s)

        chars = set()
        l = r = 0
        res = 0
        while l <= r and r < len(s):
            while s[r] in chars:
                chars.remove(s[l])
                l += 1
            else:
                res = max(res, r - l + 1)
                chars.add(s[r])
                r += 1
        return res

