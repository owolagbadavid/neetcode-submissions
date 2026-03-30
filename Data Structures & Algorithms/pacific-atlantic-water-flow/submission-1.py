class Solution:
    def pacificAtlantic(self, heights: List[List[int]]) -> List[List[int]]:
        ROWS, COLS = len(heights), len(heights[0])
        directions = [(0,1), (0,-1), (1,0), (-1,0)]
        pacific = set()
        atlantic = set()
        def dfs(r,c,mySet,visited):
            visited.add((r,c))
            mySet.add((r,c))

            for dr, dc in directions:
                nr, nc = r+dr, c+dc
                if min(nr,nc) < 0 or nr >= ROWS or nc >= COLS or heights[nr][nc] < heights[r][c] or (nr,nc) in visited:
                    continue
                dfs(nr, nc, mySet, visited)
        
        for c in range(COLS):
            dfs(0, c, pacific, set())      

        for r in range(ROWS):
            dfs(r, 0, pacific, set())      

        for c in range(COLS):
            dfs(ROWS - 1, c, atlantic, set())

        for r in range(ROWS):
            dfs(r, COLS - 1, atlantic, set())
    
        return list(pacific & atlantic)