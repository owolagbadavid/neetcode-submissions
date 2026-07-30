impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut emails = HashMap::new();
        let mut res = vec![];
        let mut uf = UF::new(accounts.len());

        for (i, a) in accounts.iter().enumerate() {
            for j in 1..a.len() {
                let e = &a[j];

                if emails.contains_key(e) {
                    uf.union(emails[e], i);
                } else {
                    emails.insert(e, i);
                }
            } 
        }

        let mut email_groups = HashMap::new();

        for (e, i) in emails {
            let leader = uf.find(i);
            let list = email_groups.entry(leader).or_insert(vec![]);
            list.push(e.clone());
        }

        for (k, mut v) in email_groups {
            let name = accounts[k][0].clone();
            v.sort();
            v.insert(0, name);
            res.push(v);
        }

        res
    }
}

#[derive(Debug)]
struct UF {
    par: Vec<usize>,
    rank: Vec<i32>
}

impl UF {
    fn new(n: usize) -> Self {
        UF {
            par: (0..n).collect(),
            rank: vec![1; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while x != self.par[x] {
            self.par[x] = self.par[self.par[x]];
            x = self.par[x];
        }
        x
    }

    fn union(&mut self, x1: usize, x2: usize) -> bool {
        let p1 = self.find(x1);
        let p2 = self.find(x2);

        if p1 == p2 {
            return false;
        }
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