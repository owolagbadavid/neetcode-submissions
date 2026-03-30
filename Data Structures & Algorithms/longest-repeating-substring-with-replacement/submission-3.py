class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        l = r = 0
        res = 1
        chars = defaultdict(int)
        fr = s[l]
        while r < len(s):
            chars[s[r]] += 1
            if chars[s[r]] > chars[fr]:
                fr = s[r]
            while r-l+1 - chars[fr] > k:
                chars[s[l]] -= 1
                l += 1
                for i in range(l, r+1):
                    if chars[s[i]] > chars[fr]:
                        fr = s[i]
            else:
                res = max(res, r-l+1)
                r += 1
        return res
