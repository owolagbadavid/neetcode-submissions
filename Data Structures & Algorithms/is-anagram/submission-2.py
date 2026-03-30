class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        cache = defaultdict(int)
        for i in s:
            cache[i] += 1
        for i in t:
            cache[i] -= 1
            if cache[i] < 0:
                return False
        for i in cache:
            if cache[i] != 0:
                return False
        return True