class TrieNode:
    def __init__(self):
        self.children = {}
        self.word = False
        self.wordIndex = 0

class Trie:
    def __init__(self):
        self.root = TrieNode()
    
    def add(self, word, index):
        cur = self.root
        for c in word:
            if c not in cur.children:
                cur.children[c] = TrieNode()
            cur = cur.children[c]
        cur.word = True
        cur.wordIndex = index

    def word(self, word, cur = None):
        if not cur:
            cur = self.root
        for c in word:
            if c not in cur.children:
                return False
            cur = cur.children[c]
        return cur.word
    
    def prefix(self, word, cur = None):
        if not cur:
            cur = self.root
        cur = self.root
        for c in word:
            if c not in cur.children:
                return False
            cur = cur.children[c]
        return True

class Solution:
    def findWords(self, board: List[List[str]], words: List[str]) -> List[str]:
        trie = Trie()

        for i in range(len(words)):
            trie.add(words[i], i)
        
        visited = set()
        res = []
        ROWS, COLS = len(board), len(board[0])
        def dfs(i, j, node):
            if (i, j) in visited or i < 0 or i >= ROWS or j < 0 or j >= COLS:
                return

            char = board[i][j]
            if node.word and node.wordIndex >= 0:
                res.append(words[node.wordIndex])
                node.wordIndex = -1
            
            visited.add((i, j))
            if i+1 < ROWS and board[i+1][j] in node.children:
                dfs(i+1, j, node.children[board[i+1][j]])
            if i-1 >= 0 and board[i-1][j] in node.children:
                dfs(i-1, j, node.children[board[i-1][j]])
            if j-1 >= 0 and board[i][j-1] in node.children:
                dfs(i, j-1, node.children[board[i][j-1]])
            if j+1 < COLS and board[i][j+1] in node.children:
                dfs(i, j+1, node.children[board[i][j+1]])

            visited.remove((i,j))
        
        for i in range(ROWS):
            for j in range(COLS):
                if board[i][j] in trie.root.children:
                    dfs(i, j, trie.root.children[board[i][j]])
        return res
            