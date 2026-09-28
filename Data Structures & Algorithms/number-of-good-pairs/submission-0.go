func numIdenticalPairs(nums []int) int {
    count := map[int]int{}
    res := 0

    for _, n := range nums {
        if val, ok := count[n]; ok {
            res += val
        }
        count[n]++
    }

    return res
}