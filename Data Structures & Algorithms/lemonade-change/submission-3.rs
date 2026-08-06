impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut ten, mut five) = (0, 0);

        for b in bills {
            if b > 5 {
                let p = b - 5;

                if five == 0 {
                    return false;
                }

                five -= 1;

                if p == 15 {
                    if ten > 0 {
                        ten -= 1;
                    } else {
                        if five < 2 {
                            return false;
                        }
                        five -= 2;
                    }
                } else {
                    ten += 1;
                }

            } else {
                five += 1
            }
        }

        true
    }
}
