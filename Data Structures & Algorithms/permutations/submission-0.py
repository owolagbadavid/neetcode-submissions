class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        res = []
        chosen = [False]*len(nums)

        def dfs(chosen, arr):
            if len(nums) == len(arr):
                res.append([*arr])
                return

            for i in range(len(nums)):
                if not chosen[i]:
                    chosen[i] = True
                    arr.append(nums[i])
                    dfs(chosen, arr)
                    chosen[i] = False
                    arr.pop()

        dfs(chosen, [])
        return res