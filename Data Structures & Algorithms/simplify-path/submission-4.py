class Solution:
    def simplifyPath(self, path: str) -> str:
        stack = []
        i = 0
        while i < len(path):
            if path[i] == "/":
                i += 1
                while i < len(path) and path[i] == "/":
                    i += 1
            else:
                start = i
                i += 1
                while i < len(path) and path[i] != "/":
                    i += 1
                cur = path[start:i]
                if cur == "..":
                    if stack:
                        stack.pop()
                elif cur != ".":
                    stack.append(path[start:i])
        return "/" + "/".join(stack)
            