mod par;
use par::Problem;
use rand::SeedableRng;
use rand_pcg::Pcg64;

fn main() {
    // Initialize random seed
    let seed = 6;
    let mut rng = Pcg64::seed_from_u64(seed);

    // Generate a problem
    let mut p = Problem::from_files("instances/zoo_set.dat", "instances/zoo_set_const_10.const", 7);
    let (results, dev) = p.greedy(&mut rng);

    println!("Results:");
    for (i, cluster) in results.iter().enumerate() {
        println!("Cluster {}: {}", i, cluster);
    }
    println!("General deviation: {}", dev);
}
