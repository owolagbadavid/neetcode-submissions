func isMonotonic(nums []int) bool {
    ind := 0

	for i := 1; i < len(nums); i += 1 {
		if nums[i] > nums[i-1] {
			if ind == -1 {
				return false
			}
			ind = 1
		} else if nums[i] < nums[i-1] {
			if ind == 1 {
				return false
			}
			ind = -1
		}
	}

	return true
}