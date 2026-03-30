class Solution:
    def orangesRotting(self, grid: List[List[int]]) -> int:
        q = deque()
        ROWS, COLS = len(grid), len(grid[0])
        fresh = rotten = 0

        for r in range(ROWS):
            for c in range(COLS):
                if grid[r][c] == 2:
                    rotten +=1
                    q.append([r,c])
                if grid[r][c] == 1:
                    fresh += 1
        
        def bfs(r, c):
            if min(r, c) < 0 or r >= ROWS or c >= COLS or grid[r][c] == 0 or grid[r][c] == 2:
                return
            nonlocal fresh
            fresh -= 1
            grid[r][c] = 2
            q.append([r,c])

        minute = 0
        while q:
            for _ in range(len(q)):
                r, c = q.popleft()
                bfs(r-1, c)
                bfs(r+1, c)
                bfs(r, c-1)
                bfs(r, c+1)
            minute += 1

        if not rotten:
            return 0 if not fresh else -1
        
        return -1 if fresh else minute - 1