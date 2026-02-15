use i_triangle::float::triangulator::Triangulator as CoreTriangulator;
use i_triangle::i_overlay::core::solver::Solver;

use super::{IntTriangulationIndex, types::IntTriangulatorValidation};

/// FFI-safe wrapper around `i_triangle::float::triangulator::Triangulator` producing `f32` results.
pub struct Float32Triangulator {
    pub(crate) inner: CoreTriangulator<IntTriangulationIndex>,
}

impl Float32Triangulator {
    #[inline]
    pub fn new(max_points_count: usize, validation: IntTriangulatorValidation) -> Self {
        let mut solver = Solver::default();
        solver.multithreading = None;
        Self {
            inner: CoreTriangulator::new(max_points_count, validation.into(), solver),
        }
    }
}
