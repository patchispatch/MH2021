// Declare sub-modules
mod problem;
mod cluster;

// Use par::<element> instead of calling par::<submodule>::<element>
pub use problem::Problem;
pub use cluster::Cluster;
