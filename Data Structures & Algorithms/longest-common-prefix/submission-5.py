class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        i = 0
        while True:
            if i > len(strs[0])-1:
                break
            cur = strs[0] 
            valid = True
            for j in range(len(strs)):
                if i > len(strs[j])-1 or strs[j][i] != cur[i]:
                    valid = False
                    break
            if not valid:
                break
            i += 1
        return strs[0][:i]

