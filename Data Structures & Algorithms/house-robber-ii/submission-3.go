func rob(nums []int) int { 
    if len(nums) == 1 {
        return nums[0]
    }
    dp1 := make([]int, len(nums)+2)
    dp2 := make([]int, len(nums)+2)

    for i := len(nums)-1; i > 0 ;i-- {
        dp1[i] = max(dp1[i+1], dp1[i+2]+nums[i])
    } 

    for i := len(nums)-2; i >= 0 ;i-- {
        dp2[i] = max(dp2[i+1], dp2[i+2]+nums[i])
    } 

    return max(dp1[1], dp2[0])
}
