class Solution:
    def combinationSum(self, nums: List[int], target: int) -> List[List[int]]:
        res = []

        def bc(index, arr, summ):
            print(arr)
            if summ == target:
                res.append([*arr])
                return
            if summ > target:
                return
            if index >= len(nums):
                return
            
            summ += nums[index]
            arr.append(nums[index])
            bc(index, arr, summ)
            summ -= nums[index]
            arr.pop()
            bc(index + 1, arr, summ)
        bc(0, [], 0)
        return res