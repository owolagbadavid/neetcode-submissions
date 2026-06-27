class Solution:
    def shipWithinDays(self, weights: List[int], days: int) -> int:
        lo, hi = max(weights), sum(weights)
        res = hi

        def can(cap):
            ship, curr = 1, 0
            for w in weights:
                if curr + w > cap:
                    curr = w
                    ship += 1
                    if ship > days:
                        return False
                else:
                    curr += w
            return True

        while lo <= hi:
            m = (hi + lo) // 2
            works = can(m)
            if works:
                res = min(m, res)
                hi = m-1
            else:
                lo = m+1

        return res