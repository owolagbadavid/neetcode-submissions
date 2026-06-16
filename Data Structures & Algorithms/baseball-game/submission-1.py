class Solution:
    def calPoints(self, operations: List[str]) -> int:
        stack = []
        for op in operations:
            if op.isdigit() or op[1:].isdigit():
                stack.append(int(op))
            elif op == '+':
                stack.append(stack[-1]+stack[-2])
            elif op == 'C':
                stack.pop()
            else:
                stack.append(2*stack[-1])
        return sum(stack)