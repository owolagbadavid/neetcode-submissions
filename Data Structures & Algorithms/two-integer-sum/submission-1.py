class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        i, my_map = 0, {} 
        for num in nums:
            if num in my_map and num + num == target:
                return [my_map[target-num], i]
            my_map[num] = i
            if target - num in my_map and my_map[target - num] != i:
                return [my_map[target-num], my_map[num]]
            i += 1