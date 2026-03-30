class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        if len(s1) > len(s2):
            return False
        freq = defaultdict(int)
        for c in s1:
            freq[c] += 1

        l = r = 0
        while r < len(s2):
            while r-l+1 and freq[s2[r]] <= 0:
                freq[s2[l]] += 1
                l += 1
            else:
                freq[s2[r]] -= 1
                r += 1
            if r-l == len(s1):
                return True
        return False
            