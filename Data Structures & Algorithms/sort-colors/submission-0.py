class Solution:
    def sortColors(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        bucket = defaultdict(int)
        for num in nums:
            bucket[num] += 1
        index = 0
        for i in range(3):
            for _ in range(bucket[i]):
                nums[index] = i
                index += 1
