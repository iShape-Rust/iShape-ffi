use super::types::IntOverlayOptions as FfiOverlayOptions;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::{Overlay, ShapeType};
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::i_float::int::point::IntPoint;
use i_overlay::i_shape::int::shape::IntShapes;

/// Wrapper around the integer overlay that provides a stable layout for FFI consumers.
pub struct IntOverlay {
    inner: Overlay,
}

/// Errors that can occur when converting raw coordinate buffers into contours.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddContourError {
    /// The coordinate buffer length is not a multiple of two.
    OddCoordinateCount,
}

impl IntOverlay {
    /// Constructs a new integer overlay with a specified segment capacity.
    #[inline]
    pub fn new(capacity: usize, options: FfiOverlayOptions) -> Self {
        let solver = Solver::default();
        Self {
            inner: Overlay::new_custom(capacity, options.into(), solver),
        }
    }

    /// Adds a contour described by `points` with the specified shape role.
    #[inline]
    pub fn add_contour(
        &mut self,
        points: &[i32],
        shape_type: ShapeType,
    ) -> Result<(), AddContourError> {
        if points.len() % 2 != 0 {
            return Err(AddContourError::OddCoordinateCount);
        }

        let iter = points
            .chunks_exact(2)
            .map(|chunk| IntPoint::new(chunk[0], chunk[1]));

        self.inner.add_path_iter(iter, shape_type);

        Ok(())
    }

    /// Executes the boolean operation and returns the resulting shapes.
    #[inline]
    pub fn overlay(&mut self, overlay_rule: OverlayRule, fill_rule: FillRule) -> IntShapes {
        self.inner.overlay(overlay_rule, fill_rule)
    }
}
