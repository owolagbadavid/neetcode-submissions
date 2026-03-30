class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        has = defaultdict(int)
        if not (len(s) == len(t)):
            return False
        for i in range(len(s)):
            has[s[i]] += 1
        for i in range(len(t)):
            if t[i] not in has or has[t[i]] == 0:
                return False
            else:
                has[t[i]] -= 1
                if has[t[i]] == 0:
                    has.pop(t[i])
        if has:
            return False
        return True