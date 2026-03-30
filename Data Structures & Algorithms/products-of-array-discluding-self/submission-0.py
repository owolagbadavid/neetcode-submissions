class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        pre, post = 1, 1
        output = [1]*len(nums)
        for i, num in enumerate(nums):
            output[i] = pre * output[i]
            pre *= num
        for i in range(len(nums) - 2, -1, -1):
            post *= nums[i+1]
            output[i] *= post
        return output