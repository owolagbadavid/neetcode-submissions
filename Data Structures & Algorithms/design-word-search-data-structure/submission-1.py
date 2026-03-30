class TrieNode:

    def __init__(self):
        self.children = {}
        self.word = False

class WordDictionary:

    def __init__(self):
        self.root = TrieNode()

    def addWord(self, word: str) -> None:
        cur = self.root
        for c in word:
            if c not in cur.children:
                cur.children[c] = TrieNode()
            cur = cur.children[c]
        cur.word = True

    def search(self, word: str, root = None) -> bool:
        if not root:
            cur = self.root
        else:
            cur = root
        for i, c in enumerate(word):
            if c == '.':
                for key in cur.children:
                    res = self.search(key + word[i+1:], cur)
                    if res:
                        return res
                return False
            elif c not in cur.children:
                return False
            cur = cur.children[c]
        return cur.word
