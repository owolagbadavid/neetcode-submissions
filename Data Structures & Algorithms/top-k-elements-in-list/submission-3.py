class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        bucket = defaultdict(list)
        mapp = defaultdict(int)

        for n in nums:
            mapp[n] += 1

        for key in mapp.keys():
            bucket[mapp[key]].append(key)

        res = []

        index = len(nums)
        remaining = k
        while remaining > 0 and index > 0:
            length = len(bucket[index])
            if length >= 1:
                for num in bucket[index]:
                    res.append(num)
                    remaining -= 1
            index -= 1

        return res
        