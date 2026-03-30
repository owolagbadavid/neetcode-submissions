class Solution:
    def minEatingSpeed(self, piles: List[int], h: int) -> int:
        up = max(piles)
        low = 1
        res = up

        while low <= up:
            m = (up + low) // 2
            h2 = h
            for p in piles:
                h2 -= math.ceil(p/m)
                if h2 < 0:
                    low = m + 1
                    break
            if h2 >= 0:
                res = min(up, m)
                up = m - 1

        return res