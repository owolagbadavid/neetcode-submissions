class Solution:
    def hasDuplicate(self, nums: List[int]) -> bool:
        has = set()
        for num in nums:
            if num in has:
                return True
            else:
                has.add(num)
        return False