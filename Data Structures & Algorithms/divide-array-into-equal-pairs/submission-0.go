func divideArray(nums []int) bool {
	count := map[int]int{}

	for _, num := range nums {
		if _, ok := count[num]; ok {
			count[num] += 1
		} else {
			count[num] = 1
		}
	}

	for _, v := range count {
		if v % 2 != 0 {
			return false
		}
	}

	return true
}