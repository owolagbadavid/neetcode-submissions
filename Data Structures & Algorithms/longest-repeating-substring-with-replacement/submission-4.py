class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        l = r = 0
        res = 1
        maxf = 0
        chars = defaultdict(int)
        while r < len(s):
            chars[s[r]] += 1
            if chars[s[r]] > maxf:
                maxf = chars[s[r]]
            while r-l+1 - maxf > k:
                chars[s[l]] -= 1
                l += 1
            else:
                res = max(res, r-l+1)
                r += 1
        return res
