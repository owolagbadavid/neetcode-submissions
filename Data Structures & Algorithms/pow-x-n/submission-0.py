class Solution:
    def myPow(self, x: float, n: int) -> float:
        if x == 0:
            return 0
        if n == 0:
            return 1

        isNeg = n < 0
        if isNeg:
            n *= -1

        res = x
        while n > 1:
            res *= x
            n -= 1

        if isNeg:
            res = 1/res
        return res