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
use rand::random;
use na::DVector;
use std::option::Option;

// Custom types
pub type Point = DVector<f64>;


/// Represents a cluster
#[derive(Clone)]
pub struct Cluster {
    centroid: Point,
    intra_cluster_dist: f64,
    elements: HashSet<usize>,
    dimension: usize,
}

impl Cluster {
    /// Creates a new empty cluster
    /// # Parameters
    /// - dim: u8 - Dimension of points in the problem
    pub fn new(dim: usize) -> Cluster {
        Cluster {
            centroid: Point::zeros(dim),
            intra_cluster_dist: 0.0,
            elements: HashSet::new(),
            dimension: dim,
        }
    }

    /// Randomizes the centroid
    pub fn randomize_centroid(&mut self) {
        let random_centroid: Vec<f64> = (0..self.dimension).map(|_| random()).collect();
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

    /// Removes and returns a value from the cluster
    /// # Arguments
    /// - e: usize - Index of an element
    pub fn take(&mut self, e: usize) -> Option<usize> {
        self.elements.take(&e)
    }

    /// Returns a reference to the elements set
    pub fn elements(&self) -> &HashSet<usize> {
        &self.elements
    }

    /// Returns a reference to the centroid
    pub fn centroid(&self) -> &Point {
        &self.centroid
    }
}

impl fmt::Display for Cluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements_to_str = self.elements.iter().fold(String::new(), |acc, &x| acc + &x.to_string() + ", "); 
        write!(f, "Cluster(centroid: {},\nintra-cluster distance: {},\nelements: [{}]", self.centroid, self.intra_cluster_dist, elements_to_str)  
    }
}