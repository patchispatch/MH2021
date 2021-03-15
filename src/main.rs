mod par;
use par::Problem;

fn main() {
    // Generate a problem
    let p = Problem::from_files("instances/zoo_set.dat", "instances/zoo_set_const_10.const", 7);
    println!("{}", p);
}
