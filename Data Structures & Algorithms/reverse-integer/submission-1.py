class Solution:
    def reverse(self, x: int) -> int:
        isNeg = x < 0
        x = abs(x)
        MAX, MIN = (2**31) - 1, -(2**31)

        res = 0
        while x:
            res = (10*res) + x % 10
            x = x // 10
        
        if isNeg:
            res *= -1
        
        if res > MAX or res < MIN:
            return 0
        return res