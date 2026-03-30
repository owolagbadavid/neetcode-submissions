class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        my_dict = defaultdict(int)
        res = []
        candidates.sort()

        def bc(index, arr, summ):
            if summ > target:
                return
            if index >= len(candidates):
                if summ == target:
                    my_dict[tuple([*arr])] += 1
                return
            
            arr.append(candidates[index])
            summ += candidates[index]
            bc(index + 1, arr, summ)
            arr.pop()

            summ -= candidates[index]
            bc(index + 1, arr, summ)
        
        bc(0, [], 0)
        for key in my_dict:
            res.append(list(key))
        return res