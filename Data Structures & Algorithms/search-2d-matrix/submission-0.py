class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        def search(arr: List[int], target: int) -> bool:
            l, r = 0, len(arr) -1 

            while l <= r:
                m = (l + r) // 2
                if arr[m] > target:
                    r = m - 1
                elif arr[m] < target:
                    l = m + 1
                else:
                    return True
            return False

        l, r = 0, len(matrix) -1 

        while l <= r:
            m = (l + r) // 2
            if matrix[m][0] > target:
                r = m - 1
            elif matrix[m][-1] < target:
                l = m + 1
            else:
                return search(matrix[m], target)
        return False

        