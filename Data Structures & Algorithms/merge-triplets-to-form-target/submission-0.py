class Solution:
    def mergeTriplets(self, triplets: List[List[int]], target: List[int]) -> bool:
        res = [False, False, False]
        for t in triplets:
            if t[0] > target[0] or t[1] > target[1] or t[2] > target[2]:
                continue
            if t[0] == target[0]:
                res[0] = True
            if t[1] == target[1]:
                res[1] = True
            if t[2] == target[2]:
                res[2] = True
        return all(res)