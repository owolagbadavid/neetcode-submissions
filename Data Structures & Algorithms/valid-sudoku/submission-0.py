class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        row = defaultdict(set)
        col = defaultdict(set)
        box = defaultdict(set)

        for r in range(len(board)):
            for c in range(len(board)):
                rKey = r + 1
                cKey = c + 1
                bKey = (r//3, c//3)
                value = board[r][c]
                # print(value)
                # print(rKey)
                # print(cKey)
                # print(bKey)
                if value == '.':
                    print('skip')
                elif value in row[rKey]:
                    print('reason r')
                    return False
                elif value in col[cKey]:
                    print('reason c')
                    return False
                elif value in box[bKey]:
                    print('reason bx')
                    return False
                # if (value in row[rKey] or value in col[cKey] or value in box[bKey]) and value != '.':
                    # print(value)
                    # print(rKey)
                    # print(cKey)
                    # print(bKey)
                    return False
                row[rKey].add(value)
                col[cKey].add(value)
                box[bKey].add(value)
        return True