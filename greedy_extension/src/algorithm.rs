pub struct Algorithm {
    pub s_box: Vec<u16>,
    pub n: usize,
    pub m: usize,
    pub domain: Vec<u16>,
    pub min_solutions: Vec<u16>,
    pub indexes: Vec<(u32, u32)>,
    pub last_merges: Vec<(usize, usize)>
}

impl Algorithm {
    pub fn new(s_box: Vec<u16>, n: usize, m: usize) -> Self{
        let domain_size = 1 << n;
        if s_box.len() != domain_size {
            panic!("S-box is of wrong length!.");
        }
        let upper_bound = 1usize << m;
        if !s_box.iter().all(|&y| (y as usize) < upper_bound) {
            panic!("S-box elements do dot respect bounds!");
        }
        let domain = (0..domain_size).map(|value| value as u16).collect();
        println!("[*] Computing DDT");      

        // let rows = 1usize << n;
        // let cols = 1usize << m;

        let mut indexes: Vec<(u32, u32)> = vec![(0u32, 0u32); 1usize << (n + m)];
        let mut min_solutions: Vec<u16> = vec![];
        let mut start: usize = 0;
        let mut prev: Option<u32> = None;
        for dx in 1..domain_size as usize {
            let mut diffs: Vec<(u16, u16)> = vec![];
            for x1 in 0..domain_size as usize {
                let x2 = x1 ^ dx;
                if x1 >= x2 {
                    continue;
                }
                let dy = s_box[x1] ^ s_box[x2]; 
                diffs.push((dy, x1 as u16));
            }
            
            diffs.sort();
            for diff in diffs.iter() {
                let key: u32 = ((dx as u32) << m) | (diff.0 as u32);
                if prev.is_none() {
                    prev = Some(key);
                    start = min_solutions.len();
                }
                else if prev.unwrap() != key {
                    indexes[prev.unwrap() as usize] = (
                        start as u32,
                        (min_solutions.len() - start) as u32
                    );
                    prev = Some(key);
                    start = min_solutions.len();
                }
                min_solutions.push(diff.1);
            }
        }
        indexes[prev.unwrap() as usize] = (
            start as u32,
            (min_solutions.len() - start) as u32
        );
        println!("[+] DDT Computed");

        let last_merges = vec![];
        return Self {s_box, n, m, domain, min_solutions, indexes, last_merges};
    }

    pub fn query_ddt(&self, diff: (u16, u16)) -> &[u16] {
        let key: u32 = ((diff.0 as u32) << self.m) | (diff.1 as u32);
        let info = self.indexes[key as usize];
        let start = info.0 as usize;
        let end = start + info.1 as usize;
        &self.min_solutions[start..end]
    }
}

