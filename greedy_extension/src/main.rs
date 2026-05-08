use std::io::{self, Read};
use std::time::Instant;
use algorithm::Algorithm;
use rand::{thread_rng, Rng};
use std::env;

pub mod dsu;
mod greedy_extension;
mod algorithm;


fn main() {
    let nruns: usize = env::args().nth(1).map(|s| s.parse().expect("N must be usize")).unwrap_or(0);

    let (s, n, m) = {
        let mut inp = String::new();
        io::stdin().read_to_string(&mut inp).unwrap();
        let mut it = inp.split_whitespace();

        let n: usize = it.next().unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();
        let len = 1usize << n;
        let bound = 1usize << m;

        let a: Vec<u16> = (0..len)
            .map(|_| {
                let x: usize = it.next().unwrap().parse().unwrap();
                assert!(x < bound);
                x as u16
            })
            .collect();
        (a, n, m)
    };

    let start = Instant::now();
    let mut alg = Algorithm::new(s.clone(), n, m);
    println!("DDT took {:?}", start.elapsed());

    let mut max_seen: usize = 0;
    let mut num_seen: usize = 0;
    let mut i: usize = 0;
    let mut limit: usize = 10;
    let mut print_interval: usize = 1;

    let mut trng = thread_rng();

    let start = Instant::now();
    loop {
        let seed: u64 = trng.gen();
        // let seed: u64 = 9247679960101856852; // for debugging
        let init: Vec<u16> = vec![];
        // let init = vec![124, 354, 388];
        let resp: Vec<u16> = greedy_extension::greedy_extension_rust(&mut alg, &init, seed);
        // let resp: Vec<usize> = greedy_extension::greedy_extension_u16(&mut alg, None);
        
        //println!("Rust greedy_extension took {:?}", elapsed);
        // println!("finished with {:?}", resp.len());
        i = i+1;
        
        if resp.len() >= max_seen {
            if resp.len() > max_seen {
                num_seen = 0;
                limit = 10;
                print_interval = 1;
            }
            max_seen = resp.len();
            num_seen += 1;
            if num_seen % print_interval == 0 {
                let time_per_itr = start.elapsed().as_secs() as f32 / i as f32;
                let itrs_per_hit = (i as f32) / (num_seen as f32);
                println!("Iter#{} New set #{} of size {} ({:.1} its/hit = {:.3}s/hit): seed={} {:?}",
                    i, num_seen, resp.len(),
                    itrs_per_hit, itrs_per_hit * time_per_itr,
                    seed, resp);
                // println!("Merges: {:?}", alg.last_merges);
            }
            if num_seen == limit {
                limit *= 10;
                print_interval *= 10;
            }
        }
        if i == nruns {
            break;
        }
        // if i >= 1 {
        // if i >= 100 { break; } 
        // if i >= 10_000 { break; } 
    }
}
