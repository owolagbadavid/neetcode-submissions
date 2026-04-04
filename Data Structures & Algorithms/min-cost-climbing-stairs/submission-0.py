class Solution:
    def minCostClimbingStairs(self, cost: List[int]) -> int:
        cache = {}

        def dfs(index, sofar):
            if (index, sofar) in cache:
                return cache[(index, sofar)]

            if index == len(cost):
                return sofar
            if index > len(cost):
                return float('inf')

            cache[(index,sofar)] = min(dfs(index+1, sofar+cost[index]), dfs(index+2, sofar+cost[index]))

            return cache[(index,sofar)] 
        return min(dfs(0,0), dfs(1,0))
