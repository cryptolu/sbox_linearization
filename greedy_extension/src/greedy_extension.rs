use std::collections::BinaryHeap;
use rand::{Rng};
use rand::seq::index;
use rand::SeedableRng;
use rand::rngs::StdRng;
use crate::dsu::RangeDSU;
use crate::algorithm::{Algorithm};
use pyo3::prelude::*;
use pyo3::exceptions::PyKeyboardInterrupt;
use std::cmp::Reverse;

const GREEDY_H: usize = 7;

pub struct GreedyExtension {
    pub n: usize,
    pub m: usize,
    pub seed: u64,
    pub rng: StdRng,

    pub ls: Vec<(u16, u16)>, // (dx, dy)
    pub lsx: Vec<u16>, // dx
    pub lsx_set: Vec<u64>,  // bitset
    
    pub cliques: RangeDSU,
    pub queue: BinaryHeap<Reverse<(usize, i16, usize)>>,

    pub cl1: usize,
}

impl GreedyExtension {
    pub fn new(alg: &Algorithm, initial: &Vec<u16>, seed: u64) -> Self {
        let n = alg.n;
        // let m = alg.m;
        
        let mut rng = StdRng::seed_from_u64(seed);
        let mut queue: BinaryHeap<Reverse<(usize, i16, usize)>> = BinaryHeap::from(
            index::sample(&mut rng, 1usize << n, GREEDY_H + 1)
                .into_iter()
                .map(|i| Reverse((1usize, rng.gen::<i16>(), i)))
                .collect::<Vec<_>>()
        );
        let cl1: usize = if !initial.is_empty() {
            assert!(initial[0] as usize <= 1usize << n);
            queue.retain(|Reverse((_, _, cl))| *cl != initial[0] as usize);
            if queue.len() > GREEDY_H {
                queue.pop();
            }
            initial[0] as usize
        } else {
            queue.pop().unwrap().0.2
        };

        let mut lsx_set: Vec<u64> = vec![0; ((1usize << n) + 63) >> 6];
        lsx_set[0] = 1;

        return Self {
            n: alg.n,
            m: alg.m,
            seed: seed,
            rng: rng,

            ls: vec![(0, 0)],
            lsx: vec![0],
            lsx_set: lsx_set,

            cliques: RangeDSU::new(1usize << n),
            queue: queue,

            cl1: cl1,
        }
    }

    pub fn select_clique_to_merge(&mut self, initial: &Vec<u16>, itr: usize) -> usize{
        let mut cl2 = self.cl1;
        let mut cl2_sz: usize = 0;
        if initial.len() >= 2 + itr {
            assert!(initial[1 + itr] as usize <= 1usize << self.n);
            cl2 = initial[1 + itr] as usize;
            cl2_sz = self.cliques.size(cl2 as u16);
            self.queue.retain(|Reverse((_, _, cl))| *cl != cl2);
        }
        else {
            // Since the heap is min-heap, we have to iterate through it
            // It does not cost much (n*H total)
            for Reverse((cl_sz, _prio, cl)) in self.queue.iter() {
                // println!("itr {}: inspecting size {} cl {}", itr, *cl_sz, *cl);
                if *cl_sz >= cl2_sz {
                    cl2_sz = *cl_sz;
                    cl2 = *cl;
                }
            }
            if cl2_sz > 0 {
                assert_eq!(
                    self.cliques.size(cl2 as u16), cl2_sz,
                    "selected cl2 {} sz {} vs {}", cl2, cl2_sz, self.cliques.size(cl2 as u16)
                );
            }
        }

        // assert_ne!(cl2_sz, 0); // failure
        if cl2_sz == 0 {
            // mini-heap failure, find merge by full iteration
            // should happen VERY rarely, increase H otherwise
            println!("WARNING: mini-heap failure step {} size {} seed {}", itr, self.queue.len(), self.seed);
            for cl in 0..1usize<<self.n {
                let cl_sz = self.cliques.size(cl as u16);
                // if !lsx.contains(&(cl1 ^ cl)){
                let index = self.cl1 ^ cl;
                if self.lsx_set[index >> 6] & (1u64 << (index & 0x3f)) == 0 {
                    // assert_eq!(lsx.contains(&(cl1 ^ cl)), false);
                    self.queue.push(Reverse((cl_sz, self.rng.gen::<i16>(), cl)));
                    if self.queue.len() > GREEDY_H {
                        self.queue.pop();
                    }
                    if cl_sz > cl2_sz {
                        cl2_sz = cl_sz;
                        cl2 = cl;
                    }
                }
            }
        }
        cl2
    }

    pub fn merge_cliques(&mut self, cl1: usize, cl2: usize) {
        let orig_size = self.cliques.size(cl1 as u16); // for sanity checks
        
        // merge into main clique here
        // avoid main coset when merging other stuff
        self.cliques.union(cl1 as u16, cl2 as u16);
        
        assert!(self.cliques.size(cl1 as u16) > orig_size, "First merge failure! Seed {}", self.seed);
    }

    pub fn prepare_merging(&mut self, cl1: usize, dx0: usize) {
        // remove potential cliques in the merged coset
        self.queue.retain(|Reverse((_, _, cl))| {
            let index = *cl ^ cl1 ^ dx0;
            // let flag = self.lsx_set[index >> 6] & (1u64 << (index & 0x3f)) == 0;
            // println!("  retain? {} {}", cl, flag);
            self.lsx_set[index >> 6] & (1u64 << (index & 0x3f)) == 0
        });

        // update LSx (useful to avoid the main merged coset via 1 check)
        let prev_size = self.lsx.len();
        for i in 0..prev_size {
            let dx = (self.lsx[i] as usize) ^ dx0;
            self.lsx.push(dx as u16);
            self.lsx_set[dx >> 6] |= 1u64 << (dx & 0x3f);
        }
    }

    pub fn merge_non_main_cliques(&mut self, alg: &Algorithm, dx0: usize, dy0: usize) {
        for (vx, vy) in self.ls.iter() {
            let dx = dx0 ^ *vx as usize;
            let dy = dy0 ^ *vy as usize;
            for u1_ in alg.query_ddt((dx as u16, dy as u16)) {
                let u1: usize = *u1_ as usize;
                let u2: usize = u1 ^ dx;
                assert!(u1 < u2); // normally, DDT should be precomputed without redundancy already
                
                // skip merging if main coset
                let index = self.cl1 ^ u1;
                if (self.lsx_set[index >> 6] & (1u64 << (index & 0x3f))) != 0 {
                    continue;
                }

                // merge cliques with check
                let uopt = self.cliques.union(u1 as u16, u2 as u16);
                if uopt.is_none() {
                    // no merge
                    continue;
                }

                // push to queue maybe
                let (u, usz) = uopt.unwrap();
                if self.queue.len() < GREEDY_H || self.queue.peek().unwrap().0.0 < usz {
                    self.queue.push(Reverse((usz, self.rng.gen::<i16>(), u as usize)));
                    if self.queue.len() > GREEDY_H {
                        self.queue.pop();
                    }
                }
            }
        }
    }

    pub fn update_ls(&mut self, dx0: usize, dy0: usize) {
        // update LS
        let prev_size = self.ls.len();
        for i in 0..prev_size {
            let (vx, vy) = self.ls[i];
            self.ls.push((vx ^ dx0 as u16, vy ^ dy0 as u16));
        }
    }

    pub fn collect_main_clique(&mut self) -> Vec<u16> {
        (0..1usize<<self.n).filter( |&i|
            self.cliques.find(i as u16) as usize == self.cl1
        ).map(|i| i as u16).collect()
    }
}

pub fn greedy_extension_rust(alg: &mut Algorithm, initial: &Vec<u16>, seed: u64) -> Vec<u16> {
    let mut GreedyExtension = GreedyExtension::new(alg, initial, seed);
    let s = &alg.s_box;

    alg.last_merges = vec![];
    for itr in 0..alg.n {
        let cl1 = GreedyExtension.cl1;
        let cl2 = GreedyExtension.select_clique_to_merge(initial, itr);
        GreedyExtension.merge_cliques(cl1, cl2);
        alg.last_merges.push((cl1, cl2));
        
        if itr == alg.n-1 {
            // finished
            break;
        }

        let dx0 = cl1 ^ cl2;
        let dy0 = s[cl1] as usize ^ s[cl2] as usize;

        GreedyExtension.prepare_merging(cl1, dx0);
        GreedyExtension.merge_non_main_cliques(&alg, dx0, dy0);
        GreedyExtension.update_ls(dx0, dy0);
    }

    GreedyExtension.collect_main_clique()
}

pub fn greedy_extension_py(py: Python, alg: &mut Algorithm, initial: &Vec<u16>, seed: u64) -> PyResult<Vec<u16>> {
    // This is a copy of the greedy_extension_rust function with the difference that the keyboard
    //  interrupt event triggered from python is checked in the main loop
    let mut GreedyExtension = GreedyExtension::new(alg, initial, seed);
    let s = &alg.s_box;

    alg.last_merges = vec![];
    for itr in 0..alg.n {
        py.check_signals()?;
        let cl1 = GreedyExtension.cl1;
        let cl2 = GreedyExtension.select_clique_to_merge(initial, itr);
        GreedyExtension.merge_cliques(cl1, cl2);
        alg.last_merges.push((cl1, cl2));
        
        if itr == alg.n-1 {
            // finished
            break;
        }

        let dx0 = cl1 ^ cl2;
        let dy0 = s[cl1] as usize ^ s[cl2] as usize;

        GreedyExtension.prepare_merging(cl1, dx0);
        GreedyExtension.merge_non_main_cliques(&alg, dx0, dy0);
        GreedyExtension.update_ls(dx0, dy0);
    }

    Ok(GreedyExtension.collect_main_clique())
}