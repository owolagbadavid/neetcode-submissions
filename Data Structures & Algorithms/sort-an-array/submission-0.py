class Solution:
    def sortArray(self, nums: List[int]) -> List[int]:
        
        def mergeSort(nums):
            if not nums:
                return []
            if len(nums) == 1:
                return nums
            
            mid = len(nums) // 2
            arr1 = mergeSort(nums[:mid])
            arr2 = mergeSort(nums[mid:])
            res = []

            i = j = 0

            while i < len(arr1) and j < len(arr2):
                if arr1[i] < arr2[j]:
                    res.append(arr1[i])
                    i += 1
                else:
                    res.append(arr2[j])
                    j += 1
            if i >= len(arr1):
                res.extend(arr2[j:])
            elif j >= len(arr2):
                res.extend(arr1[i:])
            
            return res
        
        return mergeSort(nums)