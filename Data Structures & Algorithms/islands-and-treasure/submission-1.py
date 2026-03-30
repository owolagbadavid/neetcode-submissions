class Solution:
    def islandsAndTreasure(self, grid: List[List[int]]) -> None:
        ROWS, COLS = len(grid), len(grid[0])
        directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        INF = 2147483647

        def bfs(r, c):
            visited = set()
            q = deque([(r,c,0)])

            while q:
                for i in range(len(q)):
                    row, col, level = q.popleft()
                    if row < 0 or col < 0 or row >= ROWS or col >= COLS or grid[row][col] == -1 or (row, col) in visited:
                        continue
                    for val in directions:
                        q.append((row+val[0], col+val[1], level + 1))
                    if grid[row][col]:
                        grid[row][col] = min(level, grid[row][col])
                    visited.add((row, col))
                    

        for r in range(ROWS):
            for c in range(COLS):
                if grid[r][c] == 0:
                    bfs(r, c)
            