class Solution:

    def encode(self, strs: List[str]) -> str:
        ans = ''
        for s in strs:
            res = ''
            for char in s:
                res += chr(ord(char) + 3)
            ans += f'{res}\r' 
        return ans

    def decode(self, s: str) -> List[str]:
        ans, i = [], 0
        for char in s:
            if i == 0: 
                res = ''
            if char == '\r':
                ans.append(res)
                res = ''
                continue
            res += chr(ord(char) - 3)
            i += 1
        return ans
