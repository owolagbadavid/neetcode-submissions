class Solution:
    def carFleet(self, target: int, position: List[int], speed: List[int]) -> int:
        stack = []
        post, spe = zip(*sorted(zip(position, speed), reverse=True))

        for i in range(len(post)):
            t = (target - post[i]) / spe[i]
            if stack and stack[-1] >= t:
                continue
            stack.append(t)

        return len(stack)