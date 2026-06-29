class Solution:
    def splitArray(self, nums: List[int], k: int) -> int:
        lo = max(nums)
        hi = sum(nums)
        res = hi

        def can_fit(cap):
            curr = 0
            left = k
            for num in nums:
                if curr + num > cap:
                    curr = num
                    left -= 1
                    if left == 0:
                        return False
                else:
                    curr += num
            return True

        while lo <= hi:
            m = (lo + hi) // 2
            can = can_fit(m)
            if can:
                res = m
                hi = m - 1
            else:
                lo = m + 1
        return res
