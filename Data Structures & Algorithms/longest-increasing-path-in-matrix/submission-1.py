sys.setrecursionlimit(10**6)

class Solution:
    def longestIncreasingPath(self, matrix: List[List[int]]) -> int:
        directions = [(0,-1),(0,1),(1,0),(-1,0)]
        ROWS, COLS = len(matrix), len(matrix[0])
        dp = {}
        def dfs(r, c):
            if (r,c) in dp:
                return dp[(r,c)]
            dp[(r,c)] = 0
            for dr, dc in directions:
                nr, nc = r+dr, c+dc
                if min(nr, nc) >= 0 and nr < ROWS and nc < COLS and matrix[nr][nc] > matrix[r][c]:
                    dp[(r,c)] = max(dp[(r,c)], dfs(nr, nc))
            dp[(r,c)] += 1
            return dp[(r,c)]
        
        result = 0
        for i in range(ROWS):
            for j in range(COLS):
                result = max(result, dfs(i,j))
        
        return result