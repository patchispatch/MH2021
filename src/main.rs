mod par;
use par::Cluster;

fn main() {
    let c = Cluster::new(4);
    println!("{}", c);
}
