class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        mapp = defaultdict(int)
        res = [None]*(len(nums) + 1)
        result = []
        for num in nums:
            mapp[num] += 1
        for key in mapp.keys():
            val = mapp[key]
            if res[val] is None:
                res[val] = [key]
            else:
                res[val].append(key)
        for i in range(len(nums), 0, -1):
            if res[i] is not None:
                while k > 0 and len(res[i]) > 0:
                    k-=1
                    result.append(res[i].pop())
        return result