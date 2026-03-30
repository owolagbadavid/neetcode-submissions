class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        stack = []
        for t in tokens:
            if t in '+-*/':
                n2 = stack.pop()
                n1 = stack.pop()
                stack.append(self.calc(t, n1, n2))
            else:
                stack.append(int(t))
        return stack[-1]

    def calc(self, op: str, n1: int, n2: int):
        match op:
            case '+':
                return n1 + n2
            case '*':
                return n1 * n2
            case '-':
                return n1 - n2
            case '/':
                return int(n1 / n2)

            