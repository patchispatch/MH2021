use super::super::Problem; 
use super::super::Partition;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;


/// Returns the virtual neighbourhood of a partition
/// 
/// A virtual neighbour is represented by the tuple (element, new_cluster)
/// 
/// #### Arguments
/// - partition: &Partition
/// 
/// #### Return value
/// - Vec<(usize, usize)>
fn gen_neighbourhood(partition: &Partition) -> Vec<(usize, usize)> {
    let mut neighbourhood: Vec<(usize, usize)> = Vec::new();
    for (element, current_cluster) in partition.cluster_index() {
        for cluster in (0..partition.k()).filter(|x| x != current_cluster) {
            // If length is 1, current_cluster will be empty in the neighbour
            if partition.get_cluster(*current_cluster).len() != 1 {
                neighbourhood.push((*element, cluster));
            }
        }
    }

    neighbourhood
}

/// Local search algorithm
/// - problem: &Problem - Instance of a problem
/// - rng: &Pcg64 - Random number generator
/// #### Return value
/// (final_partition: Partition, current_fitness: f64, inf: usize, deviation: f64)
pub fn local_search(problem: &Problem, rng: &mut Pcg64) -> (Partition, f64, usize, f64) {
    // Start with a greedy
    let first_partition = Partition::new_rand(problem, rng);
    let mut current_fitness = problem.fitness(&first_partition);
    
    // Some(Partition) if a best neighbour is found, None if not
    let mut final_partition = first_partition.clone();
    let mut current_partition = Some(first_partition);

    // Loop ends if no better neighbour has been found
    while current_partition.is_some() {
        // Take the value and leave None on its place
        let current = current_partition.take().unwrap();

        // Generate neighbourhood and shuffle it
        let mut neighbourhood = gen_neighbourhood(&current);
        neighbourhood.shuffle(rng);

        for (element, new_cluster) in neighbourhood {
            // Generate neighbour, and if valid, calculate fitness and compare with current partition
            let neighbour = current.gen_neighbour(element, new_cluster, problem);

            if let Some(valid) = neighbour {
                let valid_fitness = problem.fitness(&valid);
                if valid_fitness < current_fitness {
                    current_partition = Some(valid);
                    current_fitness = valid_fitness;
                    break;
                }
            }
        }

        // If current_partition is None, save current in final_partition
        if current_partition.is_none() {
            final_partition = current;
        }
    }

    // Calculate data of the final partition
    let inf = problem.calc_infeasiblity(final_partition.cluster_index());
    let deviation = problem.general_deviation(final_partition.clusters());

    (final_partition, current_fitness, inf, deviation)
}