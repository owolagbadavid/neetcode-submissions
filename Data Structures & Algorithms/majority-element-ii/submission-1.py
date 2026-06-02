class Solution:
    def majorityElement(self, nums: List[int]) -> List[int]:
        n = len(nums)
        count = defaultdict(int)
        for num in nums:
            count[num] += 1
            if len(count) > 2:
                keys = list(count.keys())
                for k in keys:
                    count[k] -= 1
                    if count[k] == 0:
                        del count[k]

        return [k for k in count if len([x for x in nums if x == k]) > n/3]
