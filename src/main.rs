mod par;
use par::Problem;
use par::algorithms::greedy;
use rand::SeedableRng;
use rand_pcg::Pcg64;

fn main() {
    // Initialize random seed
    let seed = 3;
    let mut rng = Pcg64::seed_from_u64(seed);

    // Generate a problem
    let p = Problem::from_files("instances/zoo_set.dat", "instances/zoo_set_const_10.const", 7);

    // Greedy
    let (partition, inf, dev) = greedy(&p, &mut rng);

    for (i, cluster) in partition.clusters().iter().enumerate() {
        println!("Cluster {}: {}", i, cluster);
    }

    println!("Infeasibility: {}\nGeneral deviation: {}", inf, dev);
}
