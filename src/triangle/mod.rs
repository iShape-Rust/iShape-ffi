pub mod f64_triangulation;
pub mod f64_triangulator;
pub mod int_triangulation;
pub mod int_triangulator;
pub mod types;

pub type IntTriangulationIndex = u32;

pub use f64_triangulation::FlatF64Triangulation;
pub use f64_triangulator::Float64Triangulator;
pub use int_triangulation::FlatIntTriangulation;
pub use int_triangulator::IntTriangulator;
pub use types::IntTriangulatorValidation;
