class Solution:
    def simplifyPath(self, path: str) -> str:
        stack = []
        i = 0
        while i < len(path):
            if path[i] == "/":
                i += 1
                while i < len(path) and path[i] == "/":
                    i += 1
                stack.append("/")
            else:
                start = i
                i += 1
                while i < len(path) and path[i] != "/":
                    i += 1
                cur = path[start:i]
                if cur == "..":
                    j = 0
                    while stack and j < 3:
                        stack.pop()
                        j += 1
                elif cur == ".":
                    stack.pop()
                else:
                    stack.append(path[start:i])
        res = "".join(stack).rstrip("/")
        return res if res else "/"
            