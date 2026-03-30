class Solution:
    def solveNQueens(self, n: int) -> List[List[str]]:
        res = []

        def back(left, arr):
            if left == 0:
                res.append([*arr])
                return
            
            for i in range(n):
                cur = "."*n
                cur = cur[:i] + "Q" + cur[i+1:]
                curIndex = n - left
                if all(cur != s and s.index("Q") != i - (curIndex - index) 
                    and s.index("Q") != i + (curIndex - index) for index, s in enumerate(arr)):
                    arr.append(cur)
                    back(left - 1, arr)
                    arr.pop()
        
        back(n, [])
        return res