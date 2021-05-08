use super::super::Problem; 
use super::super::Partition;
use rand::Rng;
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
fn best(population: &Vec<Partition>, p1: usize, p2: usize, problem: &Problem) -> usize {
    // TODO: implement
    unimplemented!();
}

/// Generational genetic algorithm
/// #### Arguments
/// - problem: &Problem instance of a problem
/// - pop_size: u32 - Population size
/// - rng: &mut rand_pcg::Pcg64 - Random number generator
/// 
/// Returns (final_partition: Partition, current_fitness: f64, inf: usize, deviation: f64)
pub fn generational_genetic(problem: &Problem, pop_size: u32, rng: &mut Pcg64) {

    // Generational schema parameters
    let crossovers_per_pop = 0.7 * (pop_size as f64 / 2.0);
    let mutations_per_pop = 0.1 * problem.data(0).len() as f64;
    let evaluations = 100000;

    // Step 1: random population
    let mut current_population = Partition::random_population(problem, pop_size, rng);
    let population_fitness = ();
    
    for _ in 0..evaluations {
        let mut new_population: Vec<usize> = Vec::new(); 

        // Select parent population by choosing random partitions and selecting the best
        let mut parents = Vec::new();
        for _ in 0..pop_size {
            let p1 = rng.gen_range(0..pop_size) as usize;
            let p2 = rng.gen_range(0..pop_size) as usize;
            parents.push(best(&current_population, p1, p2, problem));
        }

        println!("{:?}", parents);


        // Mutate population
        // Replace previous population (with elitism)
    }
        
    
    // Return best partition of the final population
} 