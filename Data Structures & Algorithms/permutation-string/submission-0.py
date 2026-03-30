class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        if len(s1) > len(s2):
            return False
        freq = [0]*26
        for c in s1:
            freq[ord(c) - ord('a')] += 1

        l = r = 0
        while r < len(s2):
            while r-l+1 and freq[ord(s2[r]) - ord('a')] <= 0:
                freq[ord(s2[l]) - ord('a')] += 1
                l += 1
            else:
                freq[ord(s2[r]) - ord('a')] -= 1
                r += 1
            if r-l == len(s1):
                return True
        return False
            