class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: List[str]) -> int:
        q = deque([beginWord])
        level = 1
        visit = set()

        def compareWords(word1, word2):
            count = 0
            if len(word1) != len(word2):
                return 2
            
            for i in range(len(word1)):
                if word1[i] != word2[i]:
                    count += 1
                if count == 2:
                    break
            return count

        while q:
            for _ in range(len(q)):
                word = q.popleft()
                if word == endWord:
                    return level
                
                visit.add(word)
                
                for step in wordList:
                    if compareWords(word, step) == 1 and step not in visit:
                        q.append(step)
            level += 1

        return 0
      