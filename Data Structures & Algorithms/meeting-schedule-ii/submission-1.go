/**
 * Definition of Interval:
 * type Interval struct {
 *    start int
 *    end   int
 * }
 */

func minMeetingRooms(intervals []Interval) int {
    start := []int{}
    end := []int{}

    for i := range intervals {
        start = append(start, intervals[i].start)
        end = append(end, intervals[i].end)
    }

    sort.Ints(start)
    sort.Ints(end)

    res := 0

    s := 0
    e := 0
    count := 0

    for s < len(intervals) {
        if start[s] < end[e]{
            s++
            count++
        } else {
            e++
            count--
        }
        res = max(count, res)

    }

    return res
}
