pub mod world;
pub mod coordinates;
pub mod debug;
pub mod vector;
pub mod vector_graph;
pub mod mesh;

/// Generic wrapped type for quick and easy newtype pattern
pub struct W<T>(pub T);
