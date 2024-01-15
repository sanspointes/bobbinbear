pub mod scene;
pub mod reflect_shims;
pub mod coordinates;
pub mod debug;
pub mod vector;
pub mod path;

/// Generic wrapped type for quick and easy newtype pattern
pub struct W<T>(pub T);
