class Solution:
    def shipWithinDays(self, weights: List[int], days: int) -> int:
        lo, hi = weights[-1], sum(weights)
        res = hi

        def can(cap):
            ship = 1
            curr = 0
            for w in weights:
                curr += w
                if curr > cap:
                    curr = w
                    ship += 1
                if w > cap or ship > days:
                    return False
            return True

        while lo <= hi:
            m = (hi + lo) // 2
            works = can(m)
            if works:
                res = m
                hi = m-1
            else:
                lo = m+1

        return res