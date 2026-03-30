class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        row = defaultdict(set)
        col = defaultdict(set)
        box = defaultdict(set)

        for r in range(len(board)):
            for c in range(len(board[r])):
                b = (r//3, c//3)
                val = board[r][c]
                if val == '.':
                    continue
                if val in row[r] or val in col[c] or val in box[b]:
                    return False
                row[r].add(val)
                col[c].add(val)
                box[b].add(val)
        return True