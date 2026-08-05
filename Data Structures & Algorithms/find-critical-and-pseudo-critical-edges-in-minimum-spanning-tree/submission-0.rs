impl Solution {
    pub fn find_critical_and_pseudo_critical_edges(n: i32, mut edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

        for i in 0..edges.len() {
            edges[i].push(i as i32);
        }
        edges.sort_by_key(|e| e[2]);
        let mut uf = UF::new(n);

        let mut crit = vec![];
        let mut su_crit = vec![];

        let mut mst_weight = 0;
        for edge in &edges {
            let (n1, n2, w) = (edge[0], edge[1], edge[2]);
            if uf.union(n1, n2) {
                mst_weight += w;
            }
        }

        for edge in &edges {
            let (n1, n2, w, i) = (edge[0], edge[1], edge[2], edge[3]);
            let mut weight = 0;
            uf.clear();

            for edge in &edges {
                let (v1, v2, w, j) = (edge[0], edge[1], edge[2], edge[3]);
                if i != j && uf.union(v1, v2) {
                    weight += w;
                }
            }

            if *uf.rank.iter().max().unwrap() < n || weight > mst_weight {
                crit.push(i);
            } else {
                let mut weight = w;
                uf.clear();

                uf.union(n1, n2);
                for edge in &edges {
                    let (v1, v2, w) = (edge[0], edge[1], edge[2]);
                    if uf.union(v1, v2) {
                        weight += w;
                    }
                }

                if weight == mst_weight && *uf.rank.iter().max().unwrap() == n {
                    su_crit.push(i);
                }
            }
        }

        vec![crit, su_crit]
    }
}


#[derive(Debug)]
struct UF {
    rank: Vec<i32>, 
    par: Vec<i32>
}

impl UF {
    fn new(n: i32) -> Self {
        UF {
            rank: vec![1; n as usize],
            par: (0..n).collect(),
        }
    }

    fn find(&mut self, mut n: i32) -> i32 {
        while self.par[n as usize] != n {
            self.par[n as usize] = self.par[self.par[n as usize] as usize];
            n = self.par[n as usize];
        }
        n
    }

    fn union(&mut self, n1: i32, n2: i32) -> bool {
        let (p1, p2) = (self.find(n1) as usize, self.find(n2) as usize);
        if p1 == p2 {
            return false;
        }

        if self.rank[p1] > self.rank[p2] {
            self.par[p2] = p1 as i32;
            self.rank[p1] += self.rank[p2];
        } else {
            self.par[p1] = p2 as i32;
            self.rank[p2] += self.rank[p1];
        }
        true
    }

    fn clear(&mut self) {
        for i in 0..self.rank.len() {
            self.rank[i] = 1;
            self.par[i] = i as i32;
        }
    }
}