func check(nums []int) bool {
    n := len(nums)
    d := 0

    for i, num := range nums {
        if num > nums[(i+1) % n] {
            d += 1
            if d > 1 {
                return false
            }
        }
    }

	return true
}