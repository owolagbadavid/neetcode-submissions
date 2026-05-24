func reverse(x int) int {
    max := int(math.Pow(2, 31) - 1)
    min := int(-math.Pow(2, 31))

    res := 0
    for x != 0 {
        digit := x % 10;
        if res > max / 10 || res < min / 10 {
            return 0;
        }
        if res == max / 10 && digit > max % 10 {
            return 0;
        }
        if res == min / 10 && digit < min % 10 {
            return 0;
        }
        res = (10*res) + digit;
        x = x / 10;
    }
        
    return res
}
