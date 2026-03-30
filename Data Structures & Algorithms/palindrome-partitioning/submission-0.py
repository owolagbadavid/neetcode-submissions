class Solution:
    def partition(self, s: str) -> List[List[str]]:
        res = []
    
        def isPalindrome(string):
            l, r = 0, len(string) - 1
            while l < r:
                if string[l] != string[r]:
                    return False
                l += 1
                r -= 1
            return True
            
        def back(i, arr):
            if i >= len(s):
                res.append([*arr])
                return
            
            for j in range(i, len(s)):
                print(s[i:j+1])
                if isPalindrome(s[i:j+1]):
                    arr.append(s[i:j+1])
                    back(j+1, arr)
                    arr.pop()

        back(0, [])
        return res