class Solution:
    def spiralOrder(self, matrix: List[List[int]]) -> List[int]:
        res = []

        top, bottom = 0, len(matrix)
        left, right = 0, len(matrix[0])

        while top < bottom and left < right:
            for i in range(left, right):
                res.append(matrix[top][i])
            for i in range(top+1, bottom):
                res.append(matrix[i][right-1])
            
            if top + 1 < bottom:
                for i in range(right-2, left-1, -1):
                    res.append(matrix[bottom-1][i])
            if left + 1 < right:
                for i in range(bottom-2, top, -1):
                    res.append(matrix[i][left])
            top += 1
            bottom -= 1
            left += 1
            right -= 1
        return res