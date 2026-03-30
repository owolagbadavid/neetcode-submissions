class Solution:
    def trap(self, height: List[int]) -> int:
        maxesl = [0]*len(height)
        maxesl[0] = height[0]
        maxesr = [0]*len(height)
        maxesr[len(height)-1] = height[len(height)-1]

        for i in range(1, len(height)):
            maxesl[i] = max(height[i], maxesl[i-1])
        for i in range(len(height) - 2, -1, -1):
            maxesr[i] = max(height[i], maxesr[i+1])
        
        res = 0
        for i in range(0, len(height) - 1):
            res += min(maxesl[i], maxesr[i]) - height[i]
        return res    
