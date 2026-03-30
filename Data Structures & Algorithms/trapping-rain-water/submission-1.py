class Solution:
    def trap(self, height: List[int]) -> int:
        left = [0]*len(height)
        right = [0]*len(height)
        res = 0

        for i in range(len(height)):
            cur = left[i-1] if i > 0 else height[i]
            left[i] = max(cur, height[i])
        
        for i in range(len(height) - 1, -1, -1):
            cur = right[i+1] if i < len(height) - 1 else height[i]
            right[i] = max(cur, height[i])
        
        for i in range(1, len(height) - 1):
            h = min(left[i], right[i])
            res += max(0, h - height[i])
        return res