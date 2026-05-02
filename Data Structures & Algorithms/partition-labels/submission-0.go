func partitionLabels(s string) []int {
    count := make(map[rune]int)
	res := make([]int, 0)
	for _, c := range s {
		_, ok := count[c]; if ok {
			count[c] += 1
		} else {
			count[c] = 1
		}

		// count[s]++
	}

	cur := make(map[rune]struct{})
	start := 0
	for i, c := range s {
		count[c]--
		cur[c] = struct{}{}
		if count[c] == 0 {
			delete(cur, c)
		}
		if len(cur) == 0 {
			res = append(res, i - start + 1)
			start = i + 1
		}
	}

	return res
}
