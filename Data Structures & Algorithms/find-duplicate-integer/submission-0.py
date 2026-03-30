class Solution:
    def findDuplicate(self, nums: List[int]) -> int:
        sett = set()
        for n in nums:
            if n in sett:
                return n
            sett.add(n)
        