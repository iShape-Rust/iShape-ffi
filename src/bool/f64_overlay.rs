use alloc::vec::Vec;

use super::types::Float64OverlayOptions as FfiOverlayOptions;
use i_triangle::i_overlay::core::fill_rule::FillRule;
use i_triangle::i_overlay::core::overlay::ShapeType;
use i_triangle::i_overlay::core::overlay_rule::OverlayRule;
use i_triangle::i_overlay::core::solver::Solver;
use i_triangle::i_overlay::float::overlay::FloatOverlay;
use i_triangle::i_overlay::i_float::float::point::FloatPoint;
use i_triangle::i_overlay::i_shape::base::data::{Contour, Shapes};

type Float64Point = FloatPoint<f64>;
type Float64Contour = Contour<Float64Point>;
type Float64Contours = Vec<Float64Contour>;
type Float64Shapes = Shapes<Float64Point>;

/// Wrapper around floating overlay that keeps contours until execution.
pub struct Float64Overlay {
    subject: Float64Contours,
    clip: Float64Contours,
    options: FfiOverlayOptions,
}

/// Errors that can occur when converting raw coordinate buffers into contours.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddContourError {
    /// The coordinate buffer length is not a multiple of two.
    OddCoordinateCount,
}

impl Float64Overlay {
    /// Constructs a new floating overlay with an optional contour capacity hint.
    #[inline]
    pub fn new(capacity: usize, options: FfiOverlayOptions) -> Self {
        Self {
            subject: Vec::with_capacity(capacity),
            clip: Vec::with_capacity(capacity),
            options,
        }
    }

    /// Adds a contour described by `points` with the specified shape role.
    #[inline]
    pub fn add_contour(
        &mut self,
        points: &[f64],
        shape_type: ShapeType,
    ) -> Result<(), AddContourError> {
        if points.len() % 2 != 0 {
            return Err(AddContourError::OddCoordinateCount);
        }

        if points.is_empty() {
            return Ok(());
        }

        let mut contour = Float64Contour::with_capacity(points.len() / 2);
        for chunk in points.chunks_exact(2) {
            contour.push(FloatPoint::new(chunk[0], chunk[1]));
        }

        match shape_type {
            ShapeType::Subject => self.subject.push(contour),
            ShapeType::Clip => self.clip.push(contour),
        }

        Ok(())
    }

    /// Executes the boolean operation and returns the resulting shapes.
    #[inline]
    pub fn overlay(&self, overlay_rule: OverlayRule, fill_rule: FillRule) -> Float64Shapes {
        let mut solver = Solver::default();
        solver.multithreading = None;

        let mut overlay = FloatOverlay::with_subj_and_clip_custom(
            &self.subject,
            &self.clip,
            self.options.into(),
            solver,
        );

        overlay.overlay(overlay_rule, fill_rule)
    }
}
