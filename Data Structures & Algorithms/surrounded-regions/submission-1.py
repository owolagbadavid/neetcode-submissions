class Solution:
    def solve(self, board: List[List[str]]) -> None:
        ROWS, COLS = len(board), len(board[0])
        directions = [(0,1), (0,-1), (-1,0), (1,0)]
        visited = set()
        def dfs(r,c):
            if min(r,c) < 0 or r >= ROWS or c >= COLS or (r,c) in visited:
                return
            
            if board[r][c] == 'X':
                return

            visited.add((r,c))
            
            for dr, dc in directions:
                nr, nc = dr+r, dc+c
                dfs(nr, nc)

        for r in range(ROWS):
            if board[r][0] == 'O':
                dfs(r, 0)
            if board[r][COLS-1] == 'O':
                dfs(r, COLS-1)
        for c in range(COLS):
            if board[0][c] == 'O':
                dfs(0, c)
            if board[ROWS-1][c] == 'O':
                dfs(ROWS-1, c)
        
        for r in range(ROWS):
            for c in range(COLS):
                if (r,c) not in visited:
                    board[r][c] = 'X'
            