pub struct RangeDSU {
    parent: Vec<u16>,
    size: Vec<u16>,
}

impl RangeDSU{

    pub fn new (n: usize) -> RangeDSU {
        assert!(n <= 1usize << 16);
        RangeDSU {
            parent: (0..n).map(|i| i as u16).collect(), size: vec![1; n]
        }
    }

    pub fn find(&mut self, mut a: u16) -> u16 {
        while self.parent[a as usize] != a {
            let b = self.parent[a as usize];
            self.parent[a as usize] = self.parent[b as usize];
            a = b;
        }
        a
    }

    pub fn size(&mut self, x: u16) -> usize {
        let root = self.find(x);
        // now no hash‐lookup—just direct vec index
        self.size[root as usize] as usize
    }

    pub fn union(&mut self, a: u16, b: u16) -> Option<(u16, usize)> {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb { return None }

        // make ra the larger one
        if self.size[ra as usize] < self.size[rb as usize] {
            std::mem::swap(&mut ra, &mut rb);
        }
        // attach rb → ra
        self.parent[rb as usize] = ra;
        self.size[ra as usize] += self.size[rb as usize];

        Some((ra, self.size[ra as usize] as usize))
    }
}