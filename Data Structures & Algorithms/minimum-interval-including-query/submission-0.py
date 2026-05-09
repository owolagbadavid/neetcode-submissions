class Solution:
    def minInterval(self, intervals: List[List[int]], queries: List[int]) -> List[int]:
        intervals.sort()
        heap = []
        res = {}
        index = 0
        ind = defaultdict(list)

        for i, q in enumerate(queries):
            ind[q].append(i)

        queries.sort()

        for q in queries:
            for i in range(index, len(intervals)):
                start, end = intervals[i]
                if start > q:
                    break
                if start <= q and end >= q:
                    heapq.heappush(heap, (end - start + 1, end))
                index += 1
            while heap and heap[0][1] < q:
                heapq.heappop(heap)
            if heap:
                res[q] = heap[0][0]
            else:
                res[q] = -1

        result = [-1]*len(queries)
        for k in ind:
            for i in ind[k]:
                result[i] = res[k]
        return result