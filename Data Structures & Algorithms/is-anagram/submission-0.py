class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        if len(s) != len(t):
            return False
        mapp = {};
        for ss in s:
            mapp[ss] = mapp.get(ss, 0) + 1
        for ss in t:
            mapp[ss] = mapp.get(ss, 0) - 1
            if mapp[ss] < 0:
                return False
        return sum(mapp.values()) == 0    

