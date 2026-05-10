class Solution:
    def trap(self, height: List[int]) -> int:
        n = len(height)
        res = 0
        lh = [0]*n
        rh = [0]*n
        for i in range(len(height)):
            lh[i] = max(lh[i-1], height[i]) if i > 0 else height[i]
        for i in range(len(height)-1, -1, -1):
            rh[i] = max(rh[i+1], height[i]) if i < len(height)-1 else height[i]

        for i in range(len(height)):
            res += min(lh[i], rh[i])-height[i]

        return res
