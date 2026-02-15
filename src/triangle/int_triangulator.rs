use i_triangle::i_overlay::core::solver::Solver;
use i_triangle::int::triangulator::IntTriangulator as CoreIntTriangulator;

use super::{IntTriangulationIndex, types::IntTriangulatorValidation};

/// Errors that can occur when interpreting a flat contour buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriangulateContourError {
    OddCoordinateCount,
}

/// FFI-safe wrapper around `i_triangle::int::triangulator::IntTriangulator`.
pub struct IntTriangulator {
    pub(crate) inner: CoreIntTriangulator<IntTriangulationIndex>,
}

impl IntTriangulator {
    /// Constructs a new triangulator with explicit validation configuration.
    #[inline]
    pub fn new(max_points_count: usize, validation: IntTriangulatorValidation) -> Self {
        let mut solver = Solver::default();
        solver.multithreading = None;
        Self {
            inner: CoreIntTriangulator::new(max_points_count, validation.into(), solver),
        }
    }
}
