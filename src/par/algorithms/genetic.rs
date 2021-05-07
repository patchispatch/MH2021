use super::super::Problem; 
use super::super::Partition;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;


/// Returns a new partition based on two existing ones using 
/// the **Uniform crossover operator**
fn uniform_crossover(p1: &Partition, p2: &Partition) -> Partition {
    // TODO: implement
    unimplemented!();
}

/// Returns a new partition based on two existing ones using
/// the **fixed segment crossover operator**
fn fixed_segment_crossover(p1: &Partition, p2: &Partition) -> Partition {
    // TODO: implement
    unimplemented!();
}

/// Mutates a partition using the **Uniform Mutation Operator**
fn uniform_mutation(p: &mut Partition) {
    // TODO: implement
    unimplemented!();
}

/// Selects the best partition between two and returns its index
/// #### Arguments
/// - **population: &Vec\<Partition\>** Current population
/// - **p1: usize** Index of the first partition
/// - **p2: usize** Index of the second partition
fn tournament(population: &Vec<Partition>, p1: usize, p2: usize, problem: &Problem) -> usize {
    // TODO: implement
    unimplemented!();
}
