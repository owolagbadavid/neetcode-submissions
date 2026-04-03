class Solution:
    def climbStairs(self, n: int) -> int:
        cache = {}

        def climb(num):
            if num in cache:
                return cache[num]

            if num == n:
                return 1

            if num > n:
                return 0

            cache[num] = climb(num+1) +  climb(num+2)
            return cache[num]
        return climb(0)


        