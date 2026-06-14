class Solution:
    def containsNearbyDuplicate(self, nums: List[int], k: int) -> bool:
        count = set()
        l = 0
        for r in range(len(nums)):
            while r - l > k:
                count.remove(nums[l])
                l += 1
            if nums[r] in count:
                return True
            count.add(nums[r])
        return False