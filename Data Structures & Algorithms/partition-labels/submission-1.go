func partitionLabels(s string) []int {
    count := make(map[rune]int)
	res := make([]int, 0)
	for _, c := range s {
		count[c]++
	}

	set := make(map[rune]struct{})
	start := 0

	for i, c := range s {
		count[c]--
		set[c] = struct{}{}
		if count[c] == 0 {
			delete(set, c)
		}
		if len(set) == 0 {
			res = append(res, i - start + 1)
			start = i + 1
		}
	}

	return res
}
