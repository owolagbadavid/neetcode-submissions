class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        ans, arr = {}, []
        for val in strs:
            copy = list(val)
            copy.sort()
            key = ''.join(copy)
            if key in ans:
                ans[key].append(val)
            else:
                ans[key] = [val]
        for val in ans:
            arr.append(ans[val])
        return arr