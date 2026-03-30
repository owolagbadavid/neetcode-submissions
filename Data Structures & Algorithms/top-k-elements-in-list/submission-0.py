class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        dic = defaultdict(int)
        ans = [None]*k
        for num in nums:
            dic[num] += 1
        keys, vals = list(dic.keys()), list(dic.values())
        for i in range(k):
            index = vals.index(max(vals))
            ans[i] = keys[index] 
            vals[index] = float('-inf')
            
        return ans