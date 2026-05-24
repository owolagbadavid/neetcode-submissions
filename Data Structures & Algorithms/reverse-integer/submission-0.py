class Solution:
    def reverse(self, x: int) -> int:
        isNeg = x < 0
        x = abs(x)
        m = (2**31) - 1

        res = 0
        while x:
            res = (10*res) + x % 10
            x = x // 10
        
        if isNeg:
            res *= -1
        
        if res > m or res < -m:
            return 0
        return res