class Solution:
    def majorityElement(self, nums: List[int]) -> List[int]:
        n = len(nums)
        count = defaultdict(int)
        res = set()
        for num in nums:
            count[num] += 1
            if count[num] > n/3:
                res.add(num)
        return list(res)
