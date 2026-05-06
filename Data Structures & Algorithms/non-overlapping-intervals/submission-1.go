func eraseOverlapIntervals(intervals [][]int) int {
    res := 0

	sort.Slice(intervals, func(i, j int) bool {
		if intervals[i][0] == intervals[j][0] {
			return intervals[i][1] < intervals[j][1]
		}
		return intervals[i][0] < intervals[j][0]
	})

	last := intervals[0][1]
	for i := 1; i < len(intervals); i++ {
		if last <= intervals[i][0] {
			last = intervals[i][1]
		} else {
			res++
			last = min(intervals[i][1], last)
		}
	}

	return res
}
