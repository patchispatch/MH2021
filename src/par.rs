// Declare sub-modules
mod problem;
mod partition;
mod algorithms;

// Use par::<element> instead of calling par::<submodule>::<element>
pub use problem::Problem;
pub use partition::Cluster;
pub use partition::Partition;
