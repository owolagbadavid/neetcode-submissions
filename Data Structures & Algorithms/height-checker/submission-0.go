func heightChecker(heights []int) int {
    expected := append([]int(nil), heights...)
    sort.Ints(expected)

    res := 0

    for i, h := range heights {
        if h != expected[i] {
            res += 1
        }
    }

    return res
}