//! # Cluster
//! Representation of a cluster on a PAR problem

// use statements
use std::collections::HashSet;
use std::fmt;
use rand::random;
use na::DVector;

// Custom types
pub type Point = DVector<f64>;


/// Represents a cluster
pub struct Cluster {
    centroid: Point,
    intra_cluster_dist: f64,
    elements: HashSet<u32>,
}

impl Cluster {
    /// Creates a new empty cluster
    /// # Parameters
    /// - psize: u8 - Size of points in the problem
    pub fn new(psize: usize) -> Cluster {
        let random_centroid: Vec<f64> = (0..psize).map(|_| random()).collect();

        Cluster {
            centroid: Point::from(random_centroid),
            intra_cluster_dist: 0.0, 
            elements: HashSet::new()
        }
    } 
}

impl fmt::Display for Cluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements_to_str = self.elements.iter().fold(String::new(), |acc, &x| acc + &x.to_string() + ", "); 
        write!(f, "Cluster(centroid: {},\nintra-cluster distance: {},\nelements: [{}]", self.centroid, self.intra_cluster_dist, elements_to_str)  
    }
}