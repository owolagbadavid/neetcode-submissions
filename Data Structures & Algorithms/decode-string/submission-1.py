class Solution:
    def decodeString(self, s: str) -> str:
        stack = []
        for c in s:
            if c == "]":
                cur = ""
                while stack and stack[-1] != "[":
                    cur = stack.pop() + cur
                _ = stack.pop() # ignore "["
                mul = ""
                while stack and stack[-1].isdigit():
                    mul = stack.pop() + mul
                mul = int(mul)
                stack.append(cur*mul)
            else:
                stack.append(c)

        return "".join(stack)