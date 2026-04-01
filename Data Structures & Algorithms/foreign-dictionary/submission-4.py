class Solution:
    def foreignDictionary(self, words: List[str]) -> str:
        adj = {}
        for i in range(len(words)):
            for j in range(len(words[i])):
                if words[i][j] not in adj:
                    adj[words[i][j]] = []
        for i in range(len(words)):
            if i+1 < len(words):
                word1, word2 = words[i], words[i+1]
                n = min(len(word1), len(word2))
                for j in range(n):
                    if word1[:n] == word2[:n] and len(word1) > n:
                        return ""
                    if word1[j] != word2[j]:
                        adj[word1[j]].append(word2[j])
                        break;

        visit = set()
        cycle = set()
        res = ""
        def dfs(n):
            nonlocal res
            if n in cycle:
                return True
            if n in visit:
                return False
            
            visit.add(n)
            cycle.add(n)
            
            if n in adj:
                for x in adj[n]:
                    if dfs(x):
                        return True
            res += n
            cycle.remove(n)

            return False

        for k in adj:
            if dfs(k):
                return ""

        return res[::-1]