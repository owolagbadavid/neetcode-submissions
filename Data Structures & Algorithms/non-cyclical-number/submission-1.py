class Solution:
    def isHappy(self, n: int) -> bool:
        seen = set()
        while n != 1:
            seen.add(n)
            string = str(n)
            l, r = 0, len(string)-1
            n = 0
            while l <= r:
                n += int(string[l])**2
                if r > l:
                    n += int(string[r])**2
                l+=1
                r-=1
            if n in seen:
                return False
        return True