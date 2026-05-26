class Solution:
    def getConcatenation(self, nums: List[int]) -> List[int]:
        for i in range(len(nums)):
            num = nums[i]
            nums.append(num)
        return nums