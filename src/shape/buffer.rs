use alloc::vec::Vec;
use core::ops::Range;
use i_overlay::i_float::int::point::IntPoint;
use i_overlay::i_shape::int::count::PointsCount;
use i_overlay::i_shape::int::shape::{IntContour, IntShape, IntShapes};

/// Half-open range helper that can safely cross the FFI boundary.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RangeFFI {
    pub start: u64,
    pub end: u64,
}

impl From<Range<usize>> for RangeFFI {
    #[inline]
    fn from(value: Range<usize>) -> Self {
        RangeFFI {
            start: value.start as u64,
            end: value.end as u64,
        }
    }
}

/// Flattened container for `IntShapes` data that is easy to consume from Swift.
///
/// The buffer uses three parallel arrays:
/// - `flat_points` stores the coordinates as `[x0, y0, x1, y1, ...]`.
/// - `contour_ranges` points into `flat_points`, describing the span of every contour.
/// - `shape_ranges` points into `contour_ranges`, describing which contours belong to each shape.
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct FlatShapesBuffer {
    pub flat_points: Vec<i32>,
    pub contour_ranges: Vec<RangeFFI>,
    pub shape_ranges: Vec<RangeFFI>,
}

impl FlatShapesBuffer {
    /// Constructs an empty buffer reserving the requested capacities for reuse.
    #[inline]
    pub fn with_capacity(points: usize, contours: usize, shapes: usize) -> Self {
        let mut buffer = FlatShapesBuffer::default();
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
    pub fn set_shapes(&mut self, shapes: &[IntShape]) {
        let point_count = shapes.points_count();
        let contour_count: usize = shapes.iter().map(IntShape::len).sum();
        let shape_count = shapes.len();

        self.clear_and_reserve(point_count * 2, contour_count, shape_count);

        self.push_shapes(shapes);
    }

    /// Populates the buffer from a list of shapes without clearing first.
    ///
    /// The caller is responsible for reserving enough capacity.
    #[inline]
    pub fn push_shapes(&mut self, shapes: &[IntShape]) {
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

    /// Converts the buffer back into `IntShapes`.
    #[inline]
    pub fn to_shapes(&self) -> IntShapes {
        let mut shapes: IntShapes = Vec::with_capacity(self.shape_ranges.len());
        for shape_range in &self.shape_ranges {
            let start = shape_range.start as usize;
            let end = shape_range.end as usize;
            let mut shape: IntShape = Vec::with_capacity(end - start);
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
    fn push_contour(&mut self, contour: &[IntPoint]) -> RangeFFI {
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
    fn slice_to_contour(&self, slice: &[i32]) -> IntContour {
        debug_assert!(slice.len() % 2 == 0);
        let mut contour = Vec::with_capacity(slice.len() / 2);
        for coords in slice.chunks_exact(2) {
            contour.push(IntPoint::new(coords[0], coords[1]));
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

impl From<&[IntShape]> for FlatShapesBuffer {
    #[inline]
    fn from(shapes: &[IntShape]) -> Self {
        let point_count = shapes.points_count();
        let contour_count: usize = shapes.iter().map(IntShape::len).sum();
        let shape_count = shapes.len();

        let mut buffer =
            FlatShapesBuffer::with_capacity(point_count * 2, contour_count, shape_count);
        buffer.push_shapes(shapes);
        buffer
    }
}

impl From<&IntShapes> for FlatShapesBuffer {
    #[inline]
    fn from(shapes: &IntShapes) -> Self {
        FlatShapesBuffer::from(shapes.as_slice())
    }
}
