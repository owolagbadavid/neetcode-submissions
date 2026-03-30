class Solution:
    def maxArea(self, heights: List[int]) -> int:
        l, r = 0, len(heights) - 1
        area = 0
        while l < r:
            h = min(heights[l], heights[r])
            b = r - l
            area = max(area, h*b)
            if heights[l] > heights[r]:
                r -= 1
            elif heights[r] > heights[l]:
                l += 1
            else:
                l += 1
        return area
