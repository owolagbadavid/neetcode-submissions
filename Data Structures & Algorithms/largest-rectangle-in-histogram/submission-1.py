class Solution:
    def largestRectangleArea(self, heights: List[int]) -> int:
        stack = [(0,heights[0])]
        maxArea = 0
        for i in range(1, len(heights)):
            index = i
            while stack and stack[-1][1] >= heights[i]:
                index, val = stack.pop()
                maxArea = max(maxArea, val * (i - index))
            stack.append((index, heights[i]))
        for index, val in stack:
            maxArea = max(maxArea, val * (len(heights) - index))
        return maxArea