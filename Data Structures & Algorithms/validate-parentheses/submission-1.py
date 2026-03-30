class Solution:
    def isValid(self, s: str) -> bool:
        stack = ['']
        for i, c in enumerate(s):
            if c in '}])':
                other = self.rOther(c)
                if stack[len(stack)-1] == other:
                    stack.pop()
                else:
                    return False
            else:
                stack.append(c)
        return len(stack) == 1
    
    def rOther(self, c: str):
        match c:
            case "}":
                return "{"
            case "]":
                return "["
            case ")":
                return "("
        