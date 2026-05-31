class NumMatrix:

    def __init__(self, matrix: List[List[int]]):
        COLS, ROWS = len(matrix[0]), len(matrix)
        self.hor = [[0]*(COLS+1) for _ in range(ROWS+1)]
        for r in range(ROWS):
            prefix = 0
            for c in range(COLS):
                prefix += matrix[r][c]
                self.hor[r+1][c+1] = prefix + self.hor[r][c+1]

    def sumRegion(self, row1: int, col1: int, row2: int, col2: int) -> int:
        r1, c1, r2, c2 = row1+1, col1+1, row2+1, col2+1
        end = self.hor[r2][c2]
        end -= self.hor[r1-1][c2]
        end -= self.hor[r2][c1-1]
        return end + self.hor[r1-1][c1-1]
        


# Your NumMatrix object will be instantiated and called as such:
# obj = NumMatrix(matrix)
# param_1 = obj.sumRegion(row1,col1,row2,col2)