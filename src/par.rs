// Declare sub-modules
mod problem;
mod cluster;
mod algorithms;

// Use par::<element> instead of calling par::<submodule>::<element>
pub use problem::Problem;
pub use cluster::Cluster;
