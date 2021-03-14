use na::{DVector, MatrixMN, Dynamic};
use std::vec::Vec;
use std::fs::*;

use std::io::{BufReader, BufRead};
use super::Cluster;

// Custom types
pub type Point = DVector<f64>;
pub type Column = DVector<i8>;
pub type Matrix<T> = MatrixMN<T, Dynamic, Dynamic>;


pub struct Problem {
    clusters: Vec<Cluster>,
    data: Vec<Point>,
    constraints: Matrix<i8>,
}

impl Problem {
    /// Creates a new Problem with data from two files
    /// # Arguments
    /// - data_file: &str - Path to a data file
    /// - constraints_file: &str - Path to a constraint file
    pub fn from_files(data_file: &str, constraints_file: &str) -> Problem {
        // Attributes
        let mut clu = Vec::new();
        let mut points = Vec::new();
        let mut cons: Matrix<i8>;

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

            if !c.is_empty() {
                let c: Vec<i8> = c.split(",").map(|i| i.parse().unwrap()).collect();
                columns.push(Column::from(c));  
            }
        }

        // Create Matrix from array of rows
         // Initialize constraint matrix with the problem dimensions
         cons = Matrix::from_columns(columns.as_slice());
        
        
        // TODO: generate clusters
        
        
        // Returns a Problem
        Problem {
            clusters: clu,
            data: points,
            constraints: cons,
        } 
    }
}