impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        let mut uf = UF::new(nums.len());
        let mut factor_map = HashMap::new();
        for i in 0..nums.len() {
            let factors = Self::factors(nums[i]);
            for n in factors {
                let list = factor_map.entry(n).or_insert(i);
                if *list != i {
                    uf.union(*list, i);
                }
            }
        }

        let p = uf.find(0);
        uf.rank[p] == nums.len()
    }

    fn factors(mut n: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut p = 2;

        while p*p <= n {
            if n % p == 0 {
                res.push(p);
                while n % p == 0 {
                    n /= p;
                }
            }
            
            p += 1;
        }

        if n > 1 {
            res.push(n);
        }

        res
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 {
            b
        } else if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}

#[derive(Debug)]
struct UF {
    par: Vec<usize>, rank: Vec<usize>
}

impl UF {
    fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(), 
            rank: vec![1; n],
        }
    }

    fn find(&mut self, mut n: usize) -> usize {
        while n != self.par[n] {
            self.par[n] = self.par[self.par[n]];
            n = self.par[n];
        }
        n
    }

    fn union(&mut self, n1: usize, n2: usize) -> bool {
        let (p1, p2) = (self.find(n1), self.find(n2));

        if p1 == p2 {
            false
        } else {
            if self.rank[p1] > self.rank[p2] {
                self.par[p2] = p1;
                self.rank[p1] += self.rank[p2];
            } else {
                self.par[p1] = p2;
                self.rank[p2] += self.rank[p1];
            }
            true
        }
    }
}