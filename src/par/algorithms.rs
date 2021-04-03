use super::Problem;
use super::Partition;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use std::collections::HashMap;


/// Greedy COPKM
/// - problem: &mut par::Problem - Instance of a problem
/// - rng: &mut rand_pcg::Pcg64 - Random number generator
/// #### Return value
/// (Vec<par::Cluster>, usize, f64)
/// Final partition, infeasibility and general deviation
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
                let infeasibility = problem.inf_insert(element_index, cluster, partition.clusters()); 
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

/// Local search
pub fn local_search() {
    unimplemented!();
}