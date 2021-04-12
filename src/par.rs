// Extern use statements
use serde_derive::*;

// Declare sub-modules
mod problem;
mod partition;
pub mod algorithms;

// Use par::<element> instead of calling par::<submodule>::<element>
pub use problem::Problem;
pub use partition::Cluster;
pub use partition::Partition;

// Struct for serialization
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExecutionRecord {
    pub instance: usize,
    pub aggregate: f64,
    pub infeasibility: usize,
    pub general_deviation: f64,
    pub time: u128,
}
