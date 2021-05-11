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

/// Mutates a population using the **Uniform Mutation Operator**
fn uniform_mutation(population: &mut Vec<Partition>, mutations: u32, rng: &mut Pcg64) {
    let total_elements = population[0].problem_size();
    let k = population[0].k();

    // Since cromosomes to cross are random, generate array of all indexes, shuffle and halve it
    let mut cromosomes_to_mutate: Vec<usize> = (0..population.len()).collect();
    cromosomes_to_mutate.shuffle(rng);
    cromosomes_to_mutate = cromosomes_to_mutate[..mutations as usize].to_vec();

    println!("Cromosomes to mutate: {:?}", cromosomes_to_mutate);

    for element in cromosomes_to_mutate {
        let gene_to_mutate = rng.gen_range(0..total_elements);
        let mut new_cluster = rng.gen_range(0..k);
        
        // If new cluster is the same as it was, select random cluster again
        while *population[element].get_cluster_index_for(gene_to_mutate).unwrap() == new_cluster {
            new_cluster = rng.gen_range(0..k);
        }

        // Mutate
        population[element].insert(gene_to_mutate, new_cluster); 
    }
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
    let mutations_per_pop = (0.1 * problem.len() as f64) as u32;
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
        uniform_mutation(&mut current_population, mutations_per_pop, rng);

        // Replace previous population (with elitism)
        current_population.sort_by(|a, b| {
            problem.fitness(&a).partial_cmp(&problem.fitness(&b)).unwrap()
        }); 
        let previous_best = current_population[0].clone();

        if !new_population.contains(&previous_best) {
            new_population.sort_by(|a, b| {
                problem.fitness(&b).partial_cmp(&problem.fitness(&a)).unwrap()
            });
            let new_worst = new_population[0].clone();

            // Exclude the worst, insert the best
            new_population.remove(new_population.iter().position(|x| *x == new_worst).expect("New worst not found"));
            new_population.push(previous_best.clone());
        }

        // Replace population
        current_population = new_population;
    }
    
    // Return best partition of the final population 
    current_population.sort_by(|a, b| {
        problem.fitness(&a).partial_cmp(&problem.fitness(&b)).unwrap()
    });
} 