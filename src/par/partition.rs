use super::Problem;
use std::collections::{HashSet, BTreeMap};
use std::fmt;
use std::cell::Cell;
use rand::Rng;
use na::DVector;
use rand_pcg::Pcg64;

// Custom types
pub type Point = DVector<f64>;


/// Struct to represent and manage a partition
/// - cluster_index: HashMap<usize, usize> Map to check the cluster containing an element
/// - clusters: Vec<Cluster> Vector of Cluster struct
#[derive(Clone)]
pub struct Partition {
    cluster_index: BTreeMap<usize, usize>,
    clusters: Vec<Cluster>,
    fitness: Cell<Option<f64>>
}


impl Partition {
    /// Creates a new empty Partition with random centroids for each cluster
    /// - k: usize - Number of clusters in the partition
    /// - dim: usize - Dimension of a point in the problem
    pub fn new(k: usize, dim: usize, rng: &mut Pcg64) -> Partition {
        let mut clu = Vec::new();
        for _ in 0..k { 
            clu.push(Cluster::new_rand(dim, rng));
        }

        Partition {
            cluster_index: BTreeMap::new(),
            clusters: clu,
            fitness: Cell::new(None)
        }
    }

    /// Creates a new random valid Partition given an instance of Problem
    /// - problem: &Problem - Instance of a problem
    /// - rng: &mut rand_pcg::Pcg64 - Random number generator
    pub fn new_rand(problem: &Problem, rng: &mut Pcg64) -> Partition {
        let dim = problem.data(0).len();

        let mut partition = Partition {
            cluster_index: BTreeMap::new(),
            clusters: vec![Cluster::new(dim); problem.k()],
            fitness: Cell::new(None)
        };

        for (index, _) in problem.get_data().iter().enumerate() {
            let random_cluster = rng.gen_range(0..problem.k());
            partition.insert(index, random_cluster);
        }

        // Check if valid. If not, launch recursively
        if partition.clusters.iter().filter(|x| x.len() == 0).next().is_some() {
            partition = Partition::new_rand(problem, rng);
        }

        partition
    }

    /// Creates a population of new random partitions with *n* elements
    /// - problem: &Problem - Instance of a problem
    /// - n: u32 - number of elements of the partition
    /// - rng: &mut rand_pcg::Pcg64 - Random number generator
    pub fn random_population(problem: &Problem, n: u32, rng: &mut Pcg64) -> Vec<Partition> {
        let mut population = Vec::new();

        for _ in 0..n {
            population.push(Partition::new_rand(problem, rng));
        }

        population
    }

    /// Returns total number of elements (size of the problem)
    pub fn problem_size(&self) -> usize {
        self.cluster_index.len()
    }

    /// Insert an element into a cluster
    /// - element: usize - Index of element to insert
    /// - cluster: usize - Index of cluster 
    pub fn insert(&mut self, element: usize, cluster: usize) {
        // If the element is in another cluster, remove it
        if self.cluster_index.contains_key(&element) {
            self.clusters[cluster].remove(element);
        }
        
        // Insert in the new cluster and update the index
        self.clusters[cluster].insert(element);
        self.cluster_index.insert(element, cluster);

        // Set the fitness buffer to None
        self.fitness.set(None);
    }

    /// [DEPRECATED] Insert an element into a cluster and update its centroid 
    /// - element: usize - Index of element to insert
    /// - cluster: usize - Index of cluster 
    /// - problem: &Problem - Instance of the problem (needed to calculate new cluster centroid)
    pub fn insert_and_update(&mut self, element: usize, cluster: usize, problem: &Problem) {
        // If the element is in another cluster, remove it
        if self.cluster_index.contains_key(&element) {
            self.clusters[cluster].remove(element);
        }
        
        // Insert in the new cluster and update the index
        self.clusters[cluster].insert_and_update(element, problem);
        self.cluster_index.insert(element, cluster);

        // Set the fitness buffer to None
        self.fitness.set(None);
    }

    /// Generate a neighbour by changing `element` to `cluster`
    /// #### Return value:
    /// - `Some(neighbour)` where neighbour is valid
    /// - `None` if the neighbour is not valid
    pub fn gen_neighbour(&self, element: usize, cluster: usize, problem: &Problem) -> Option<Partition> {
        let mut neighbour = self.clone();
        neighbour.insert_and_update(element, cluster, problem);
        
        // Check if valid
        if neighbour.get_cluster(cluster).is_empty() {
            None
        }
        else {
            Some(neighbour)
        }
    }

    /// Checks if a partition is valid
    /// A partition is considered valid when there is no empty clusters inside it
    pub fn is_valid(&self) -> bool {
        if self.clusters.iter().filter(|x| x.is_empty()).next().is_none() {
            true
        }
        else {
            false
        }
    }

    /// Randomly repairs an invalid partition, moving an element to its empty clusters
    /// #### Arguments
    /// - rng: &mut Pcg64 - Random number generator
    pub fn repair(&mut self, rng: &mut Pcg64) {
        let clusters_to_repair: Vec<usize> = self.clusters
            .iter()
            .enumerate()
            .filter(|&(_, c)| c.is_empty())
            .map(|(i, _)| i)
            .collect();

        for cluster in clusters_to_repair { 
            // Pick an element and check if its cluster will still be valid
            let mut random_element = rng.gen_range(0..self.problem_size());
            while self.clusters[*self.get_cluster_index_for(random_element).unwrap()].len() <= 1 {
                random_element = (random_element + 1) % self.problem_size();
            }

            // Repair invalid cluster 
            self.clusters[cluster].insert(random_element);
        }
    }

    /// Return number of clusters
    pub fn k(&self) -> usize {
        self.clusters.len()
    }

    /// Get reference to cluster index
    pub fn cluster_index(&self) -> &BTreeMap<usize, usize> {
        &self.cluster_index
    }

    /// Get reference to clusters vector
    pub fn clusters(&self) -> &Vec<Cluster> {
        &self.clusters
    }

    /// Get a cluster reference from an index
    /// - i: usize - Cluster index
    pub fn get_cluster(&self, i: usize) -> &Cluster {
        &self.clusters[i]
    }

    /// Get a mutable reference of a cluster from an index
    /// - i: usize - Cluster index
    pub fn get_cluster_mut(&mut self, i: usize) -> &mut Cluster {
        &mut self.clusters[i]
    }

    /// Get value of cluster index by a key
    /// - element: usize - Element to check
    /// Returns *None* if not in the index
    pub fn get_cluster_index_for(&self, element: usize) -> Option<&usize> {
        self.cluster_index.get(&element)
    }

    /// Return fitness of the partition
    /// - problem: &Problem - Instance of the problem
    pub fn fitness(&self, problem: &Problem) -> f64 {
        if let Some(fit) = self.fitness.get() {
            fit
        }
        else {
            let fit = problem.fitness(self);
            self.fitness.set(Some(fit));
            fit
        }
    }
}

impl PartialEq for Partition {
    fn eq(&self, other: &Self) -> bool {
        self.cluster_index == other.cluster_index 
    }
}


/// Representation of a cluster on a PAR problem
/// ## Elements
/// - centroid: Point - Current centroid
/// - elements: HashSet<usize> - Set of element indexes (not Points)
/// - dimension: usize - Dimension of the problem Points
#[derive(Clone)]
pub struct Cluster {
    centroid: Point,
    elements: HashSet<usize>,
    dimension: usize,
}

impl Cluster {
    /// Creates a new empty cluster
    /// # Parameters
    /// - dim: usize - Dimension of points in the problem
    pub fn new(dim: usize) -> Cluster {
        Cluster {
            centroid: Point::zeros(dim),
            elements: HashSet::new(),
            dimension: dim,
        }
    }

    /// Creates a new empty cluster with a randomized centroid
    /// # Arguments
    /// - dim: usize - Dimension of points in the problem
    pub fn new_rand(dim: usize, rng: &mut Pcg64) -> Cluster {
        let mut new_cluster = Cluster {
            centroid: Point::zeros(dim),
            elements: HashSet::new(),
            dimension: dim,
        };
        new_cluster.randomize_centroid(rng);
        
        new_cluster
    }

    /// Randomizes the centroid
    pub fn randomize_centroid(&mut self, rng: &mut Pcg64) {
        let random_centroid: Vec<f64> = (0..self.dimension).map(|_| rng.gen()).collect(); 
        self.centroid = Point::from(random_centroid);
    }

    /// Sets a new centroid
    /// # Arguments
    /// - c: Point - The new centroid
    pub fn set_centroid(&mut self, c: Point) {
        self.centroid = c;
    }

    /// Inserts a new element into the cluster
    /// # Arguments
    /// - e: usize - Index of an element
    pub fn insert(&mut self, e: usize) -> bool {
        self.elements.insert(e)
    }

    /// Inserts a new element into the cluster and updates the centroid
    /// # Arguments
    /// - e: usize - Index of an element
    pub fn insert_and_update(&mut self, e: usize, problem: &Problem) -> bool {
        let success = self.elements.insert(e);

        if success {
            self.centroid = problem.calc_centroid(self);
        }

        success
    }

    /// Removes an element from the cluster
    /// # Arguments
    /// - e: usize - Index of an element
    pub fn remove(&mut self, e: usize) -> bool {
        self.elements.remove(&e)
    }

    /// Returns true if a cluster contains an element
    /// # Arguments
    /// - e: usize - Index of an element
    pub fn contains(&self, e: usize) -> bool {
        self.elements.contains(&e)
    }

    /// Returns a reference to the elements set
    pub fn elements(&self) -> &HashSet<usize> {
        &self.elements
    }

    /// Returns a reference to the centroid
    pub fn centroid(&self) -> &Point {
        &self.centroid
    }

    /// Returns dimension of the cluster
    pub fn dimension(&self) -> usize {
        self.dimension
    }

    /// Returns `true` if cluster is empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Returns the number of elements in the cluster
    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

impl fmt::Display for Cluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements_to_str = self.elements.iter().fold(String::new(), |acc, &x| acc + &x.to_string() + ", "); 
        write!(f, "Cluster(centroid: {},\nelements: [{}]", self.centroid, elements_to_str)
    }
}