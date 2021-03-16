//! # Cluster
//! Representation of a cluster on a PAR problem
//! ## Elements
//! - centroid: Point - Current centroid
//! - intra_cluster_distance: f64 - Saved intra-cluster distance
//! - elements: HashSet<u32> - Set of element indexes (not Points)
//! - size: usize - Dimension of the problem

// use statements
use std::collections::HashSet;
use std::fmt;
use rand::random;
use na::DVector;
use std::option::Option;

// Custom types
pub type Point = DVector<f64>;


/// Represents a cluster
pub struct Cluster {
    centroid: Point,
    intra_cluster_dist: f64,
    elements: HashSet<usize>,
    size: usize,
}

impl Cluster {
    /// Creates a new empty cluster
    /// # Parameters
    /// - psize: u8 - Size of points in the problem
    pub fn new(psize: usize) -> Cluster {
        Cluster {
            centroid: Point::zeros(psize),
            intra_cluster_dist: 0.0, 
            elements: HashSet::new(),
            size: psize,
        }
    }

    /// Randomizes the centroid
    fn randomize_centroid(&self) {
        let random_centroid: Vec<f64> = (0..self.size).map(|_| random()).collect();
        self.centroid = Point::from(random_centroid);
    }

    /// Sets a new centroid
    /// # Arguments
    /// - c: Point - The new centroid
    fn set_centroid(&self, c: Point) {
        self.centroid = c;
    }

    /// Inserts a new element into the cluster
    /// # Arguments
    /// - e: usize - Index of an element
    fn insert(&self, e: usize) -> bool {
        self.elements.insert(e)
    }

    /// Removes an element from the cluster
    /// # Arguments
    /// - e: usize - Index of an element
    fn remove(&self, e: usize) -> bool {
        self.elements.remove(&e)
    }

    /// Removes and returns a value from the cluster
    /// # Arguments
    /// - e: usize - Index of an element
    fn take(&self, e: usize) -> Option<usize> {
        self.elements.take(&e)
    }
}

impl fmt::Display for Cluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements_to_str = self.elements.iter().fold(String::new(), |acc, &x| acc + &x.to_string() + ", "); 
        write!(f, "Cluster(centroid: {},\nintra-cluster distance: {},\nelements: [{}]", self.centroid, self.intra_cluster_dist, elements_to_str)  
    }
}

// TODO: implement Clone trait
/*
impl std::clone::Clone for Cluster {
    
}
*/