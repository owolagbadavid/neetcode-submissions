class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        mapp = {}
        for stri in strs:
            arr = [0]*26
            for char in stri:
                arr[ord(char) - 97] += 1
            key = str(arr)
            if key in mapp:
                mapp[key].append(stri)
            else:
                mapp[key] = [stri]
        return list(mapp.values())
