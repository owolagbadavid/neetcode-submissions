class Solution:
    def canJump(self, nums: List[int]) -> bool:
        n = len(nums)
        dp = [False]*n
        dp[n-1] = True

        for i in range(n-2, -1, -1):
            for j in range(1, nums[i]+1):
                dp[i] = dp[j+i] if j+i < n else True
                if dp[i]:
                    break
        
        return dp[0]