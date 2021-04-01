use par::Problem;
use rand_pcg::Pcg64;


/// Greedy COPKM
/// #### Arguments
/// - problem: &mut par::Problem - Instance of a problem
/// - rng: &mut rand_pcg::Pcg64 - Random number generator
/// #### Return value
/// (Vec<par::Cluster>, usize, f64)
/// Final partition, infeasibility and general deviation
pub fn greedy(problem: &mut Problem, rng: &mut Pcg64) -> () {
    // Step 1: create k empty clusters with a random centroid
    let dimension = problem.data(0).len();
    self.clusters = (0..self.k).map(|_| Cluster::new_rand(dimension, rng)).collect();

    // Step 2: Shuffle element indexes
    self.data.shuffle(rng);

    // Step 3: while there are changes in clustering
    let mut changes = true;
    while changes {
        changes = false;
        
        // Step 4: for every element
        for (i, item) in self.data.iter().enumerate() {
            // Calculate infeasibility increment of assigning to each cluster
            let mut cl_inf = HashMap::new();
            let mut min_inf = usize::MAX;

            for c in 0..self.k {
                let inf_for_c = self.inf_insert(i, c);
                cl_inf.insert(c, inf_for_c);

                // If infeasibility increment is below the current minimum, update it
                if inf_for_c < min_inf {
                    min_inf = inf_for_c;
                }
            } 

            // Of the clusters with lesser infeasibility increment, select the nearest and insert the element
            let mut candidates: Vec<usize> = cl_inf.iter().filter(|x| *x.1 == min_inf).map(|(index, _)| *index).collect();
            candidates.sort_by(|a, b| {
                item.metric_distance(self.clusters[*a].centroid()).partial_cmp(&item.metric_distance(self.clusters[*b].centroid())).unwrap()
            });
            let best = candidates[0];
            
            // If element is not already on the cluster, insert it and mark that changes has been made
            if !self.clusters[best].contains(i) {
                Problem::insert_into_cluster(&mut self.clusters, i, best);
                changes = true;
            }
        }

        // Step 4: for every cluster
        for c in 0..self.k {
            // Calculate new centroid with the assigned elements
            let centroid = self.calc_centroid(c); 
            self.clusters[c].set_centroid(centroid);
        }
    }

    // Calculate infeasibility of the partition
    self.calc_infeasiblity();


    // Return partition as a Vec<Cluster>
    (self.clusters.clone(), self.general_deviation())
}