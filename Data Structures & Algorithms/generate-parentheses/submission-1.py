class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        res = []
        stack = []

        def back(count):
            if count[1] > count[0] or count[1] > n or count[0] > n:
                return
            if n == count[1] == count[0]:
                res.append("".join(stack))
                return
            
            count[0] += 1
            stack.append("(")
            back(count)
            count[0] -= 1
            stack.pop()

            count[1] += 1
            stack.append(")")
            back(count)
            count[1] -= 1
            stack.pop()
        
        back([0,0])
        return res