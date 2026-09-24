func isArraySpecial(nums []int) bool {
    i := 0
    j := 1

    for j < len(nums) {
        if nums[i] % 2 == nums[j] % 2 {
            return false
        }
        i += 1
        j += 1
    }

    return true
}