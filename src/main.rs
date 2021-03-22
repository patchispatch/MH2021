mod par;
use par::Problem;
use rand::SeedableRng;
use rand_pcg::Pcg64;

fn main() {
    // Initialize random seed
    let mut seed = 3;
    let rng = Pcg64::seed_from_u64(seed);

    // TODO: pass rng to algorithms as a reference and keep sharing that reference
    

    // Generate a problem
    let p = Problem::from_files("instances/zoo_set.dat", "instances/zoo_set_const_10.const", 7);
}
