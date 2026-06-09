class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        res = []
        for i in range(len(nums)):
            if i+1 < len(nums) and nums[i] == nums[i+1]:
                continue
            res.append(nums[i])
        k = len(res)
        for i in range(k):
            nums[i] = res[i]
        return k