/**
 * Definition of Interval:
 * type Interval struct {
 *    start int
 *    end   int
 * }
 */

func canAttendMeetings(intervals []Interval) bool {
    sort.Slice(intervals, func(i, j int) bool {
        if intervals[i].start == intervals[j].start {
            return intervals[i].end < intervals[j].end
        }
        return intervals[i].start < intervals[j].start
    })

    if len(intervals) <= 1 {
        return true
    }

    last := intervals[0].end
    for _, val := range intervals[1:] {
        start := val.start
        end := val.end
        if last > start {
            return false
        }
        last = max(end, last)
    }
    return true
}
