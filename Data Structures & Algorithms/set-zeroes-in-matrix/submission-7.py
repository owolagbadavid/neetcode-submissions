class Solution:
    def setZeroes(self, matrix: List[List[int]]) -> None:
        topRow = 1
        leftCol = 1
        for i in range(len(matrix)):
            for j in range(len(matrix[0])):
                if matrix[i][j] == 0:
                    if i == 0:
                        topRow = 0
                    if j == 0:
                        leftCol = 0

                    matrix[0][j] = 0
                    matrix[i][0] = 0

        for i in range(1, len(matrix)):
            for j in range(1, len(matrix[0])):
                if  matrix[0][j] == 0 or matrix[i][0] == 0:
                    matrix[i][j] = 0

        if leftCol == 0:
            for i in range(len(matrix)):
                matrix[i][0] = 0
        
        if topRow == 0:
            for i in range(len(matrix[0])):
                matrix[0][i] = 0
