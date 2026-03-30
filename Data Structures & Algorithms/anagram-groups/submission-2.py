class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        mapp = defaultdict(list)
        for stri in strs:
            arr = [0]*26
            for letter in stri:
                arr[ord(letter) - 97] += 1
            mapp[str(arr)].append(stri)
        return list(mapp.values())
