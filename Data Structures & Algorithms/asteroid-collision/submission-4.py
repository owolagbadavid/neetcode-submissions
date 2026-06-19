class Solution:
    def asteroidCollision(self, asteroids: List[int]) -> List[int]:
        stack = []
        def collide(stack):
            if len(stack) > 1 and stack[-2] > 0 and stack[-1] < 0:
                b = stack.pop()
                a = stack.pop()
                survive = a if abs(a) > abs(b) else b if abs(b) > abs(a) else None
                if survive:
                    stack.append(survive)
                collide(stack)
        for a in asteroids:
            stack.append(a)
            collide(stack)
        return stack