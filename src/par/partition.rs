use super::Problem;
use std::collections::{HashSet, HashMap, BTreeMap};
use std::fmt;
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
            clusters: clu
        }
    }

    /// Insert an element into a cluster
    /// - element: usize - Index of element to insert
    /// - cluster: usize - Index of cluster 
    /// - problem: &Problem - Instance of the problem (needed to calculate new cluster centroid)
    pub fn insert(&mut self, element: usize, cluster: usize, problem: &Problem) {
        // If the element is in another cluster, remove it
        if self.cluster_index.contains_key(&element) {
            self.clusters[cluster].remove(element);
        }
        
        // Insert in the new cluster and update the index
        self.clusters[cluster].insert(element, problem);
        self.cluster_index.insert(element, cluster);
    }

    /// Generate a neighbour by changing `element` to `cluster`
    /// #### Return value:
    /// - `Some(neighbour)` where neighbour is valid
    /// - `None` if the neighbour is not valid
    pub fn gen_neighbour(&self, element: usize, cluster: usize, problem: &Problem) -> Option<Partition> {
        let mut neighbour = self.clone();
        neighbour.insert(element, cluster, problem);
        
        // Check if valid
        if neighbour.get_cluster(cluster).is_empty() {
            None
        }
        else {
            Some(neighbour)
        }
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
    pub fn insert(&mut self, e: usize, problem: &Problem) -> bool {
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
}

impl fmt::Display for Cluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements_to_str = self.elements.iter().fold(String::new(), |acc, &x| acc + &x.to_string() + ", "); 
        write!(f, "Cluster(centroid: {},\nelements: [{}]", self.centroid, elements_to_str)
    }
}