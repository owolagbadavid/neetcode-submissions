class Solution:
    def minWindow(self, s: str, t: str) -> str:
        if len(t) > len(s):
            return ""

        l = r = 0
        has = defaultdict(int)
        has2 = defaultdict(int)
        res = ""
        tSet = set(list(t))

        for i in range(len(t)):
            has[t[i]] += 1
        for i in range(len(t)):
            if s[i] in has:
                has2[s[i]] += 1

        matches = 0
        for k in has.keys():
            matches += 1 if has2[k] >= has[k] else 0
        
        if matches == len(tSet):
            res = s[0:len(t)] if len(t) < len(res) or res == "" else res

        l = 0
        for r in range(len(t), len(s)):
            if s[r] in has:
                has2[s[r]] += 1
                matches += 1 if has2[s[r]] == has[s[r]] else 0
            while matches == len(tSet):
                res = s[l:r+1] if r-l < len(res) or res == "" else res
                if s[l] in has:
                    match = has2[s[l]] >= has[s[l]]
                    has2[s[l]] -= 1
                    matches -= 1 if has2[s[l]] < has[s[l]] and match else 0
                l += 1
        return res 
