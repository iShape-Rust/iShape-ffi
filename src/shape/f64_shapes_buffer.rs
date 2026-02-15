use alloc::vec::Vec;
use i_triangle::i_overlay::i_float::float::point::FloatPoint;
use i_triangle::i_overlay::i_shape::base::data::{Contour, Shape, Shapes};
use i_triangle::i_overlay::i_shape::float::count::PointsCount as FloatPointsCount;

use super::int_shapes_buffer::RangeFFI;

type Float64Point = FloatPoint<f64>;
type Float64Contour = Contour<Float64Point>;
type Float64Shape = Shape<Float64Point>;
type Float64Shapes = Shapes<Float64Point>;

/// Flattened container for `Float64Shapes` data that is easy to consume from Swift.
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct FlatF64ShapesBuffer {
    pub flat_points: Vec<f64>,
    pub contour_ranges: Vec<RangeFFI>,
    pub shape_ranges: Vec<RangeFFI>,
}

impl FlatF64ShapesBuffer {
    /// Constructs an empty buffer reserving the requested capacities for reuse.
    #[inline]
    pub fn with_capacity(points: usize, contours: usize, shapes: usize) -> Self {
        let mut buffer = FlatF64ShapesBuffer::default();
        buffer.reserve(points, contours, shapes);
        buffer
    }

    /// Returns `true` when no shapes are stored.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.flat_points.is_empty()
    }

    /// Removes all stored data while preserving the current capacity.
    #[inline]
    pub fn clear(&mut self) {
        self.flat_points.clear();
        self.contour_ranges.clear();
        self.shape_ranges.clear();
    }

    /// Clears the buffer while keeping enough capacity for the provided shapes.
    #[inline]
    pub fn set_shapes(&mut self, shapes: &[Float64Shape]) {
        let point_count = FloatPointsCount::points_count(shapes);
        let contour_count: usize = shapes.iter().map(Float64Shape::len).sum();
        let shape_count = shapes.len();

        self.clear_and_reserve(point_count * 2, contour_count, shape_count);

        self.push_shapes(shapes);
    }

    /// Populates the buffer from a list of shapes without clearing first.
    ///
    /// The caller is responsible for reserving enough capacity.
    #[inline]
    pub fn push_shapes(&mut self, shapes: &[Float64Shape]) {
        let mut contour_offset = self.contour_ranges.len();

        for shape in shapes {
            let shape_start = contour_offset;
            for contour in shape {
                let range = self.push_contour(contour);
                self.contour_ranges.push(range);
                contour_offset += 1;
            }

            let shape_range = RangeFFI {
                start: shape_start as u64,
                end: contour_offset as u64,
            };
            self.shape_ranges.push(shape_range);
        }
    }

    /// Converts the buffer back into `Float64Shapes`.
    #[inline]
    pub fn to_shapes(&self) -> Float64Shapes {
        let mut shapes: Float64Shapes = Vec::with_capacity(self.shape_ranges.len());
        for shape_range in &self.shape_ranges {
            let start = shape_range.start as usize;
            let end = shape_range.end as usize;
            let mut shape: Float64Shape = Vec::with_capacity(end - start);
            for contour_index in start..end {
                let contour_range = self.contour_ranges[contour_index];
                let start = contour_range.start as usize;
                let end = contour_range.end as usize;
                let slice = &self.flat_points[start..end];
                shape.push(self.slice_to_contour(slice));
            }
            shapes.push(shape);
        }

        shapes
    }

    #[inline]
    fn push_contour(&mut self, contour: &[Float64Point]) -> RangeFFI {
        let start = self.flat_points.len();
        self.flat_points.reserve(contour.len() * 2);

        for point in contour {
            self.flat_points.push(point.x);
            self.flat_points.push(point.y);
        }

        RangeFFI {
            start: start as u64,
            end: self.flat_points.len() as u64,
        }
    }

    #[inline]
    fn slice_to_contour(&self, slice: &[f64]) -> Float64Contour {
        debug_assert!(slice.len() % 2 == 0);
        let mut contour = Vec::with_capacity(slice.len() / 2);
        for coords in slice.chunks_exact(2) {
            contour.push(FloatPoint::new(coords[0], coords[1]));
        }
        contour
    }

    #[inline]
    fn clear_and_reserve(&mut self, points: usize, contours: usize, shapes: usize) {
        self.clear();
        self.flat_points.reserve(points);
        self.contour_ranges.reserve(contours);
        self.shape_ranges.reserve(shapes);
    }

    #[inline]
    fn reserve(&mut self, points: usize, contours: usize, shapes: usize) {
        self.flat_points.reserve(points);
        self.contour_ranges.reserve(contours);
        self.shape_ranges.reserve(shapes);
    }
}

impl From<&[Float64Shape]> for FlatF64ShapesBuffer {
    #[inline]
    fn from(shapes: &[Float64Shape]) -> Self {
        let point_count = FloatPointsCount::points_count(shapes);
        let contour_count: usize = shapes.iter().map(Float64Shape::len).sum();
        let shape_count = shapes.len();

        let mut buffer =
            FlatF64ShapesBuffer::with_capacity(point_count * 2, contour_count, shape_count);
        buffer.push_shapes(shapes);
        buffer
    }
}

impl From<&Float64Shapes> for FlatF64ShapesBuffer {
    #[inline]
    fn from(shapes: &Float64Shapes) -> Self {
        FlatF64ShapesBuffer::from(shapes.as_slice())
    }
}
