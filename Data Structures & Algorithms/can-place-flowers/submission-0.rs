impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        for i in 0..flowerbed.len() {
            if flowerbed[i] == 0 && flowerbed[i.saturating_sub(1)] == 0 && flowerbed[(i+1).min(flowerbed.len()-1)] == 0 {
                flowerbed[i] = 1;
                n -= 1;
            }
        }

        n <= 0
    }
}
