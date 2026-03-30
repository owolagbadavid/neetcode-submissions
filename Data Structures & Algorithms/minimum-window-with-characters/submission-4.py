class Solution:
    def minWindow(self, s: str, t: str) -> str:
        map1 = defaultdict(int)
        map2 = defaultdict(int)
        res = ""
        resL = float('infinity')

        for c in t:
            map1[c] += 1

        need = len(map1.keys())
        has = 0

        l = 0
        for r in range(len(s)):
            map2[s[r]] += 1
            if map2[s[r]] == map1[s[r]]:
                has += 1
            while need == has:
                cur = s[l:r+1]
                if len(cur) < resL:
                    res = cur
                    resL = len(cur)
                map2[s[l]] -= 1
                if map2[s[l]] < map1[s[l]]:
                    has -= 1
                l += 1

        return res
        