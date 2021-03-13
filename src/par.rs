mod problem;
mod cluster;

// Use par::<element> instead of calling submodules
pub use problem::Problem;
pub use cluster::Cluster;