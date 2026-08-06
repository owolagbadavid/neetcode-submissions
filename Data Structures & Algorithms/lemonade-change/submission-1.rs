impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut change: HashMap<i32, i32> = [5, 10, 20].map(|b| (b, 0)).into_iter().collect();

        for b in bills {
            if b > 5 {
                let p = b - 5;
                let fives = *change.get(&5).unwrap();
                let tens = *change.get(&10).unwrap();
                if fives == 0 {
                    return false;
                }
                *change.get_mut(&5).unwrap() -= 1;

                if p == 15 {
                    if tens > 0 {
                        let tens = change.get_mut(&10).unwrap();
                        if *tens == 0 {
                            return false;
                        }
                        *tens -= 1;
                    } else {
                        let fives = change.get_mut(&5).unwrap();
                        if *fives < 2 {
                            return false;
                        }
                        *fives -= 1;
                    }
                }

            }
            *change.get_mut(&b).unwrap() += 1;
        }

        true
    }
}
