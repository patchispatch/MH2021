use na::DVector;
use std::vec::Vec;
use std::collections::{HashMap, BTreeMap};
use std::fs::*;
use std::io::{BufReader, BufRead};
use std::fmt::{Display, Formatter, Result};
use colored::*;
use super::{Partition, Cluster};

// Custom types
pub type Point = DVector<f64>;


pub struct Problem {
    data: Vec<Point>,
    constraints: HashMap<(usize, usize), i8>,
    k: usize,
    lambda: f64,
}

impl Problem {
    /// Creates a new Problem with data from two files
    /// # Arguments
    /// - data_file: &str - Path to a data file
    /// - constraints_file: &str - Path to a constraint file
    pub fn from_files(data_file: &str, constraints_file: &str, cl_number: usize) -> Problem {
        // Attributes
        let mut points = Vec::new();
        let mut cons = HashMap::new();

        // Open data file
        let data = File::open(data_file).expect("Data file not found");
        let reader = BufReader::new(data);

        // Each line in the data file represents a Point in the problem space
        print!("Reading data from {}: ", data_file);
        for line in reader.lines() {
            let p = line.unwrap();
            
            if !p.is_empty() {
                let p: Vec<f64> = p.split(",").map(|i| i.parse().unwrap()).collect();
                points.push(Point::from(p));
            }
        }

        // Read data OK
        print!("{}\n", "OK".green().bold());

        // Open constraints file
        let constraints = File::open(constraints_file).expect("Constraints file not found");
        let reader = BufReader::new(constraints);

        // The constraints file represents the constraint matrix
        print!("Reading constraints from {}: ", constraints_file);

        let mut constraint_number = 0;

        for (ln, line) in reader.lines().enumerate() {
            let c = line.unwrap();

            if !c.is_empty() {
                for (i, val) in c.split(",").map(|x| x.parse::<i8>().unwrap()).enumerate().skip(ln) {
                    cons.insert((ln, i), val);
                    constraint_number += 1;

                    // Symmetric matrix
                    if ln != i {
                        cons.insert((i, ln), val);
                    }
                }
            }
        }

        // Read constraints OK
        print!("{}\n", "OK".green().bold());

        // Calculate and store distance between points
        let mut dist = HashMap::new();
        for (i, e1) in points.iter().enumerate() {
            for (j, e2) in points.iter().enumerate() {
                dist.insert((i, j), e1.metric_distance(e2));

                if i != j {
                    dist.insert((j, i), *dist.get(&(i, j)).unwrap());
                }
            }
        }

        // Calculate lambda as (max_distance + n) / |constraints|
        let lmbd = dist.values().cloned().fold(0./0., f64::max) / constraint_number as f64;

        // Returns a Problem
        Problem {
            data: points,
            constraints: cons,
            k: cl_number,
            lambda: lmbd,
        }
    }

    /// Returns a point given an index
    /// - index: usize - Index of the data vector
    /// Returns an immutable reference to a `Point`
    pub fn data(&self, index: usize) -> &Point {
        &self.data[index]
    }

    /// Returns k
    pub fn k(&self) -> usize {
        self.k
    }

    /// Returns a copy of the data vector
    pub fn get_data(&self) -> Vec<Point> {
        self.data.clone()
    }

    /// Returns value of lambda
    pub fn lambda(&self) -> f64 {
        self.lambda
    }
 
    /// Given a cluster, returns its intra-cluster distance
    /// # Arguments
    /// - clu: &Cluster - Cluster to calculate
    pub fn intra_cluster_distance(&self, cluster: &Cluster) -> f64 {
        // Accumulate distances
        let dist = cluster.elements()
            .iter()
            .fold(0.0, |acc, &x| acc + cluster.centroid().metric_distance(self.data.get(x).unwrap()));

        // Return mean
        dist / cluster.elements().len() as f64
    }

    /// Returns the general deviation of the current partition
    pub fn general_deviation(&self, clusters: &Vec<Cluster>) -> f64 {
        // Accumulate distances
        let deviation = clusters.iter()
            .fold(0.0, |acc, x| acc + self.intra_cluster_distance(&x));

        // Return mean
        deviation / self.k as f64
    }

    /// Returns the infeasibility increment of inserting an element into a cluster 
    /// #Arguments
    /// - element: i32 - Index of an element
    /// - clu: &Cluster - Cluster to check
    pub fn inf_insert(&self, element: usize, new_cluster: usize, cluster_index: &BTreeMap<usize, usize>) -> usize {
        let mut inf = 0;

        for ((_, second), con_value) in self.constraints.iter().filter(|((first, _), _)| *first == element) {
            let cannot_link = *con_value == -1 && cluster_index.get(&second) == Some(&new_cluster);
            let must_link = *con_value == 1 && cluster_index.get(&second) != Some(&new_cluster);
            if cannot_link || must_link {
                inf += 1;
            }
        }
        
        // Return value
        inf
    }

    /// Calculates the new centroid of a cluster
    /// Generates the mean point of the cluster based on the current elements
    pub fn calc_centroid(&self, cluster: &Cluster) -> Point {
        cluster.elements().iter().fold(Point::zeros(cluster.dimension()), |acc, x| acc + &self.data[*x]) / cluster.elements().len() as f64
    }

    /// Calculates the infeasibility of a given partition
    pub fn calc_infeasiblity(&self, cluster_index: &BTreeMap<usize, usize>) -> usize {
        let mut inf = 0;

        for constraint in self.constraints.iter() {
            let (e1, e2) = constraint.0;
            if e1 > e2 {
                match constraint.1 {
                    1 if cluster_index.get(e1).unwrap() != cluster_index.get(e2).unwrap() => {
                        inf += 1;
                    },
                    -1 if cluster_index.get(e1).unwrap() == cluster_index.get(e2).unwrap() => {
                        inf += 1;
                    },
                    _ => {}
                }
            }
        }

        inf
    }

    /// Returns the fitness of a given partition
    /// - partition: &Partition - Partition to evaluate
    pub fn fitness(&self, partition: &Partition) -> f64 {
        self.general_deviation(partition.clusters()) + 
            self.calc_infeasiblity(partition.cluster_index()) as f64 * self.lambda()
    }
}

// Display trait
impl Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = format!("Problem{{\n\tData: [\n\t{0} elements\n],\nConstraints: [\n\t{1} constraints\n]\n}}",
            self.data.len(),
            self.constraints.len(),
        );

        write!(f, "{}", s)
    }
}