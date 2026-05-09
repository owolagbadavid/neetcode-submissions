class Solution:
    def minInterval(self, intervals: List[List[int]], queries: List[int]) -> List[int]:
        intervals.sort()
        heap, res = [], {}
        i = 0

        for q in sorted(queries):
            while i < len(intervals):
                start, end = intervals[i]
                if start > q:
                    break
                if start <= q and end >= q:
                    heapq.heappush(heap, (end - start + 1, end))
                i += 1
            while heap and heap[0][1] < q:
                heapq.heappop(heap)
            if heap:
                res[q] = heap[0][0]
            else:
                res[q] = -1

        result = []
        for q in queries:
            result.append(res[q])
        return result