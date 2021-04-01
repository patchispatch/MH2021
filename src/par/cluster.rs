//! # Cluster
//! Representation of a cluster on a PAR problem
//! ## Elements
//! - centroid: Point - Current centroid
//! - intra_cluster_distance: f64 - Saved intra-cluster distance
//! - elements: HashSet<u32> - Set of element indexes (not Points)
//! - dimension: usize - Dimension of the problem

// use statements
use std::collections::HashSet;
use std::fmt;
use rand::Rng;
use na::DVector;
use rand_pcg::Pcg64;

// Custom types
pub type Point = DVector<f64>;


/// Represents a cluster
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
}

impl fmt::Display for Cluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements_to_str = self.elements.iter().fold(String::new(), |acc, &x| acc + &x.to_string() + ", "); 
        write!(f, "Cluster(centroid: {},\nelements: [{}]", self.centroid, elements_to_str)
    }
}