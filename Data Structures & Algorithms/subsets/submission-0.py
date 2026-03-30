class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        res = []
        def bc(index, arr):
            if index == len(nums):
                res.append([*arr])
                return

            arr.append(nums[index])
            bc(index + 1, arr)
            arr.pop()
            bc(index + 1, arr)

        bc(0, [])
        return res
        