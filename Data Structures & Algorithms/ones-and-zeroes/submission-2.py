class Solution:
    def findMaxForm(self, strs: List[str], m: int, n: int) -> int:
        count = [[0]*2 for _ in strs]
        for i in range(len(strs)):
            for j in range(len(strs[i])):
                if strs[i][j] == '0':
                    count[i][0] += 1
                else:
                    count[i][1] += 1
        
        def dfs(i, m, n):
            if (i,m,n) in cache:
                return cache[(i,m,n)]
            if i >= len(strs):
                return 0

            m1, n1 = count[i][0], count[i][1]

            res = dfs(i+1, m, n)
            if min(m-m1, n-n1) >= 0:
                res = max(res, 1 + dfs(i+1, m-m1, n-n1))
            cache[(i,m,n)] = res
            return cache[(i,m,n)]
    
        cache = {}

        return dfs(0, m, n)
        return res