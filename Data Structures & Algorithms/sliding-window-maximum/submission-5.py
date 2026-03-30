class Solution:
    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:
        l = 1
        max_heap = []
        heapq.heapify(max_heap) 
        for i in range(k):
            heapq.heappush(max_heap, (-nums[i], i))   
        res = [-max_heap[0][0]]
        for r in range(k, len(nums)): 
            heapq.heappush(max_heap, (-nums[r], r))
            index = max_heap[0][1]
            while index < l:
                heapq.heappop(max_heap)
                index = max_heap[0][1]
            res.append(-max_heap[0][0])
            l += 1
        return res