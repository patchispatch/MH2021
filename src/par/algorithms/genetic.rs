use super::super::Problem; 
use super::super::Partition;
use rand::Rng;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use std::collections::HashSet;


/// Returns a new partition based on two existing ones using 
/// the **Uniform crossover operator**
fn uniform_crossover(p1: &Partition, p2: &Partition, rng: &mut Pcg64) -> Partition {
    let total_elements = p1.problem_size();

    // Since genes to cross are random, generate array of all indexes, shuffle and halve it
    let mut genes_to_cross: Vec<usize> = (0..total_elements).collect();
    genes_to_cross.shuffle(rng);
    genes_to_cross = genes_to_cross[..total_elements/2].to_vec();
    

    // Initialize the child as a clone of p1
    let mut child = p1.clone();

    for element in genes_to_cross {
        child.insert(element, *p2.get_cluster_index_for(element).unwrap());
    }

    child
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

/// Selects the best partition between two and returns a copy of it
/// #### Arguments
/// - **p1: &'a partition** First partition
/// - **p2: &'a partition** Second partition
fn best<'a>(p1: &'a Partition, p2: &'a Partition, problem: &Problem) -> Partition {
    if problem.fitness(p1) >= problem.fitness(p2) {
        p1.clone()
    }
    else {
        p2.clone()
    }
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
    let crossovers_per_pop = (0.7 * (pop_size as f64 / 2.0)) as u32; 
    let mutations_per_pop = 0.1 * problem.data(0).len() as f64;
    let evaluations = 100;

    // Step 1: random population
    let mut current_population = Partition::random_population(problem, pop_size, rng);
    let population_fitness = ();
    
    for _ in 0..evaluations {
        let mut new_population: Vec<Partition> = Vec::new(); 

        // Select parent population by choosing random partitions and selecting the best
        let mut parents = Vec::new();
        for _ in 0..pop_size {
            let p1 = rng.gen_range(0..pop_size) as usize;
            let p2 = rng.gen_range(0..pop_size) as usize;
            parents.push(best(&current_population[p1], &current_population[p2], problem));
        }

        // Generate new population by crossing parents
        for i in (0..parents.len()).step_by(2) {
            let p1 = &parents[i];
            let p2 = &parents[i+1];

            if i/2 < crossovers_per_pop as usize {
                new_population.push(uniform_crossover(p1, p2, rng)); 
                new_population.push(uniform_crossover(p1, p2, rng));
            }
            else {
                new_population.push(p1.clone());
                new_population.push(p2.clone());
            }
        }

        // Mutate population
        

        // Replace previous population (with elitism)
    }
        
    
    // Return best partition of the final population
} 