class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        res = [1]*len(nums)
        pre = post = 1
        for i in range(0, len(nums)):
            res[i] *= pre
            pre *= nums[i]
        post = nums[len(nums)-1]
        for i in range(len(nums)-2, -1, -1):
            res[i] *= post 
            post *= nums[i]
        return res
