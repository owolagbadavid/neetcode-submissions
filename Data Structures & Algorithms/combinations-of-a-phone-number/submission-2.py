class Solution:
    def letterCombinations(self, digits: str) -> List[str]:
        if not digits:
            return []
    
        res = []
        string = ""
        my_dict = {
            "2": ["a", "b", "c"],
            "3": ["d", "e", "f"],
            "4": ["g", "h", "i"],
            "5": ["j", "k", "l"],
            "6": ["m", "n", "o"],
            "7": ["p", "q", "r", "s"],
            "8": ["t", "u", "v"],
            "9": ["w", "x", "y", "z"]
        }

        def back(index):
            nonlocal string
            if index >= len(digits):
                res.append(string[:])
                return
            
            for i in range(4):
                if len(my_dict[digits[index]]) > i:
                    string += my_dict[digits[index]][i]
                    back(index + 1)
                    string = string[:-1]
        back(0)
        return res