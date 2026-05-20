extern crate alloc;

pub mod bool;
pub mod shape;
pub mod triangle;

mod ffi_offset;
mod ffi_overlay;
mod ffi_shapes;
mod ffi_triangulation;

pub use crate::bool::{
    Float64Overlay, Float64OverlayOptions, IntContourDirection, IntFillRule, IntOverlay,
    IntOverlayOptions, IntOverlayRule, IntShapeType,
};
pub use crate::shape::{FlatF64ShapesBuffer, FlatShapesBuffer, RangeFFI};
pub use crate::triangle::{
    FlatF64Triangulation, FlatIntTriangulation, Float64Triangulator, IntTriangulationIndex,
    IntTriangulator, IntTriangulatorValidation,
};
