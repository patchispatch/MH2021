use na::{DVector, MatrixMN, Dynamic};
use std::vec::Vec;
use std::fs::*;
use std::io::{BufReader, BufRead};
use std::fmt::{Display, Formatter, Result};

use super::Cluster;

// Custom types
pub type Point = DVector<f64>;
pub type Column = DVector<i8>;
pub type Matrix<T> = MatrixMN<T, Dynamic, Dynamic>;


pub struct Problem {
    clusters: Vec<Cluster>,
    data: Vec<Point>,
    constraints: Matrix<i8>,
    infeasibility: usize,
    k: usize,
}

impl Problem {
    /// Creates a new Problem with data from two files
    /// # Arguments
    /// - data_file: &str - Path to a data file
    /// - constraints_file: &str - Path to a constraint file
    pub fn from_files(data_file: &str, constraints_file: &str, cl_number: usize) -> Problem {
        // Attributes
        let mut clu = Vec::new();
        let mut points = Vec::new();
        let cons: Matrix<i8>;
        let inf: usize = 0;

        // Open data file
        let data = File::open(data_file).expect("Data file not found");
        let reader = BufReader::new(data);

        // Each line in the data file represents a Point in the problem space
        println!("Reading file {}", data_file);
        for line in reader.lines() {
            let p = line.unwrap();
            
            if !p.is_empty() {
                let p: Vec<f64> = p.split(",").map(|i| i.parse().unwrap()).collect();
                points.push(Point::from(p));
            }
        }

        // Read data OK
        println!("Reading file {}: OK", data_file);

        // Open constraints file
        let constraints = File::open(constraints_file).expect("Constraints file not found");
        let reader = BufReader::new(constraints);

        // The constraints file represents the constraint matrix
        println!("Reading file {}", constraints_file);

        let mut columns = Vec::new();
        for line in reader.lines() {
            let c = line.unwrap();

            // TODO: implementar map

            if !c.is_empty() {
                let c: Vec<i8> = c.split(",").map(|i| i.parse().unwrap()).collect();
                columns.push(Column::from(c));  
            }
        }

        // Create Matrix from array of rows
        // Initialize constraint matrix with the problem dimensions
        cons = Matrix::from_columns(columns.as_slice());

        // Create clusters
        let point_size = points[0].len();
        for _ in 0..cl_number {
            clu.push(Cluster::new(point_size));
        }

        // Returns a Problem
        Problem {
            clusters: clu,
            data: points,
            constraints: cons,
            infeasibility: inf,
            k: cl_number,
        }
    }

    /// Returns a solution with the greedy COPKM
    /// Returns a vector of clusters
    fn greedy(&self) -> Vec<Cluster> {
        self.clusters.clone()
    }

    /// Given a cluster, returns its intra-cluster distance
    /// # Arguments
    /// - clu: &Cluster - Cluster to calculate
    fn intra_cluster_distance(&self, clu: &Cluster) -> f64 {
        // Accumulate distances
        let dist = clu.elements()
            .iter()
            .fold(0.0, |acc, &x| acc + clu.centroid().metric_distance(self.data.get(x).unwrap()));

        // Return mean
        dist / clu.elements().len() as f64
    }

    /// Returns the general deviation of the current partition
    fn general_deviation(&self) -> f64 {
        // Accumulate distances
        let deviation = self.clusters.iter()
            .fold(0.0, |acc, x| acc + self.intra_cluster_distance(&x));

        // Return mean
        deviation / self.k as f64
    }

    /// Returns the infeasibility increment of inserting an element into a cluster 
    /// using a constraint matrix
    /// #Arguments
    /// - element: i32 - Index of an element
    /// - clu: &Cluster - Cluster to check
    /// # Return value
    /// Infeasibiility increment introducing the value
    fn inf_insert(&self, element: usize, clu: usize) -> usize {
        let mut inf = 0;

        for (cl_i, cl) in self.clusters.iter().enumerate() {
            // Cannot link
            if cl_i == clu {
                for &i in cl.elements() {
                    if self.constraints[(i, element)] == -1 {
                        inf += 1;
                    }
                }
            }
            // Must link
            else {
                for &i in cl.elements() {
                    if self.constraints[(i, element)] == 1 {
                        inf += 1;
                    }
                }
            }
        }
        
        // Return value
        inf
    }


}

// Display trait
impl Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = format!("Problem{{\n\tClusters: [\n{0}\n],\nData: [\n\t{1} elements\n],\nConstraints: [\n\t{2} constraints\n]\n}}",
            self.clusters.iter().fold(String::new(), |acc, x| acc + &x.to_string() + ", \n"),
            self.data.len(),
            self.constraints.len(),
        );

        write!(f, "{}", s)
    }
}