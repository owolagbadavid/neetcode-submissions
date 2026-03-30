class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        
        l = r = 0
        dic = defaultdict(int)
        res = 0

        while r < len(s):
            dic[s[r]] += 1
            length = r - l + 1
            maxChar = max(dic.values())
            while length - maxChar > k:
                dic[s[l]] -= 1
                maxChar = max(dic.values())
                l += 1
                length = r - l + 1

            res = max(res, length)
            r += 1

        return res
