class Solution:
    def firstMissingPositive(self, nums: List[int]) -> int:
        n = len(nums)

        for i in range(n):
            nums[i] = 0 if nums[i] < 0 else nums[i]

        for i, num in enumerate(nums):
            index = abs(num) - 1
            if index < 0 or index > n - 1:
                continue
            nums[index] = -abs(nums[index]) if nums[index] != 0 else -abs(num)

        for i in range(1, n+1):
            if nums[i-1] < 0:
                continue
            return i

        return n + 1