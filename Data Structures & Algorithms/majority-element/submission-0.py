class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        n = len(nums)
        count = defaultdict(int)
        for num in nums:
            count[num] += 1
            if count[num] > n // 2:
                return num