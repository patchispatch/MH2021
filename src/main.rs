mod par;
use par::Problem;
use par::algorithms::greedy;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use std::time::Instant;

fn main() {
    // Initialize random seed
    let seed = 6;
    let mut rng = Pcg64::seed_from_u64(seed);

    // Generate a problem
    let p = Problem::from_files("instances/bupa_set.dat", "instances/bupa_set_const_20.const", 7);

    // Greedy
    let now = Instant::now();
    let (partition, inf, dev) = greedy(&p, &mut rng);
    println!("Greedy: {} milliseconds", now.elapsed().as_millis());
}
