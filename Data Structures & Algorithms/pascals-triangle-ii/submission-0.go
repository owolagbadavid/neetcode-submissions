func getRow(rowIndex int) []int {
    res := []int{1}

	for i := 1; i <= rowIndex; i++ {
		next := make([]int, i, i+1)
		next[0] = 1
		next = append(next, 1)

		for j := 1; j < i; j++ {
			next[j] = res[j] + res[j-1]
		}
		res = next
	}

	return res
}