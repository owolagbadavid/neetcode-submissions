class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        visited = set()
        res = False
        def back(i, j, index):
            nonlocal res
            if i >= len(board) or j >= len(board[0]) or (i,j) in visited:
                return
            if i < 0 or j < 0:
                return
            if board[i][j] != word[index]:
                return
            if index == len(word)-1:
                res = True
                return
            visited.add((i,j))
            
            back(i+1, j, index+1)
            back(i-1, j, index+1)
            back(i, j+1, index+1)
            back(i, j-1, index+1)

            visited.remove((i, j))
        
        for i in range(len(board)):
            for j in range(len(board[i])):
                back(i, j, 0)
                if res:
                    break
                
        return res