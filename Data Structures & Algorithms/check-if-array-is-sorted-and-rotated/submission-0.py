class Solution:
    def check(self, nums: List[int]) -> bool:
        n = len(nums)
        i = min(enumerate(nums), key=lambda x: x[1])[0]
        last = -1

        end = i + n;
        while i < end:
            if nums[i % n] < last:
                return False
            last = nums[i % n]
            i += 1

        return True


        