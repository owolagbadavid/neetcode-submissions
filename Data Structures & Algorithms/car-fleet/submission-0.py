class Solution:
    def carFleet(self, target: int, position: List[int], speed: List[int]) -> int:
        sortedPosition = sorted(enumerate(position), key=lambda x: x[1], reverse=True)
        stack = []
        for p in sortedPosition:
            time = (target - p[1]) / speed[p[0]]
            if stack and stack[-1] >= time:
                pass
            else:
                stack.append(time)
        return len(stack)

