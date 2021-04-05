use super::Problem;
use super::Partition;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use std::collections::HashMap;


/// Greedy COPKM
/// - problem: &mut par::Problem - Instance of a problem
/// - rng: &mut rand_pcg::Pcg64 - Random number generator
/// #### Return value
/// (Partition, usize, f64) Final partition, infeasibility and general deviation
pub fn greedy(problem: &Problem, rng: &mut Pcg64) -> (Partition, usize, f64) {
    // Step 1: create k empty clusters with a random centroid
    let dimension = problem.data(0).len();
    let mut partition = Partition::new(problem.k(), dimension, rng);

    // Step 2: Shuffle element indexes
    let mut data = problem.get_data();
    data.shuffle(rng);

    // Step 3: while there are changes in clustering
    let mut changes = true;
    while changes {
        changes = false;
        
        // Step 4: for every element
        for (element_index, element) in data.iter().enumerate() {
            // Calculate infeasibility increment of assigning to each cluster
            let mut cluster_infeasibility = HashMap::new();
            let mut min_infeasibility = usize::MAX;

            for cluster in 0..problem.k() {
                let infeasibility = problem.inf_insert(element_index, cluster, partition.cluster_index()); 
                cluster_infeasibility.insert(cluster, infeasibility);

                // If infeasibility increment is below the current minimum, update it
                if infeasibility < min_infeasibility {
                    min_infeasibility = infeasibility;
                }
            } 

            // Of the clusters with lesser infeasibility increment, select the nearest and insert the element
            let mut candidates: Vec<usize> = cluster_infeasibility.iter()
                .filter(|x| *x.1 == min_infeasibility).map(|(index, _)| *index)
                .collect();
            
            candidates.sort_by(|a, b| {
                element.metric_distance(partition.get_cluster(*a).centroid())
                    .partial_cmp(&element.metric_distance(partition.get_cluster(*b).centroid()))
                    .unwrap()
            });
            let best = candidates[0];
            
            match partition.get_cluster_index_for(element_index) {
                Some(current_cluster) if *current_cluster == best => {},
                _ => {
                    partition.insert(element_index, best);
                    changes = true;
                }
            }
        }

        // Step 4: for every cluster
        for c in 0..problem.k() {
            // Calculate new centroid with the assigned elements
            let centroid = problem.calc_centroid(partition.get_cluster(c)); 
            partition.get_cluster_mut(c).set_centroid(centroid);
        }
    }

    // Calculate infeasibility of the partition
    let partition_inf = problem.calc_infeasiblity(partition.cluster_index());


    // Return partition as a Vec<Cluster>
    (partition.clone(), partition_inf, problem.general_deviation(partition.clusters())) 
}

/// Local search algorithm
/// - problem: &Problem - Instance of a problem
/// - rng: &Pcg64 - Random number generator
/// #### Return value
/// I don't know
pub fn local_search(problem: &Problem, rng: &mut Pcg64) -> (Partition, usize, f64) {
    // Objective function
    let fitness = |partition: &Partition| -> f64 {
        problem.general_deviation(partition.clusters()) + 
            problem.calc_infeasiblity(partition.cluster_index()) as f64 * problem.lambda()
    };

    // Neighbourhood operator
    let gen_neighbourhood = |partition: &Partition| -> Vec<(usize, usize)> {
        let mut neighbourhood: Vec<(usize, usize)> = Vec::new();
        for (element, current_cluster) in partition.cluster_index() {
            for cluster in (0..problem.k()).filter(|x| x != current_cluster) { 
                neighbourhood.push((*element, cluster));
            }
        }

        neighbourhood
    };

    // Algorithm

    // Start with a greedy
    let (first_partition, _, _) = greedy(problem, rng);
    let mut current_fitness = fitness(&first_partition);
    
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
            let neighbour = current.gen_neighbour(element, new_cluster);

            if let Some(valid) = neighbour {
                let valid_fitness = fitness(&valid);
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

    (final_partition, inf, deviation)
}