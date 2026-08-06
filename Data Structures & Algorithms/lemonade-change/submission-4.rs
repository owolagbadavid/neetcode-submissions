impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut ten, mut five) = (0, 0);

        for b in bills {
            match b > 5 {
                true => {
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
                }

                false => {
                    five += 1
                }
            }
        }

        true
    }
}
