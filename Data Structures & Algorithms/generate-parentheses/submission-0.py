class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        res = []

        def back(count, string):
            if count[1] > count[0] or count[1] > n or count[0] > n:
                return
            if len(string) == n*2 and count[1] == count[0]:
                res.append(string)
                return
            
            count[0] += 1
            string += "("
            back(count, string)
            count[0] -= 1
            string = string[:-1]

            count[1] += 1
            string += ")"
            back(count, string)
            count[1] -= 1
            string = string[:-1]
        
        back([0,0], "")
        return res