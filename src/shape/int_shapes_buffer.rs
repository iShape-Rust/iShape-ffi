use alloc::vec::Vec;
use core::ops::Range;
use i_triangle::i_overlay::i_float::int::point::IntPoint;
use i_triangle::i_overlay::i_shape::flat::buffer::FlatShapesBuffer as CoreFlatShapesBuffer;
use i_triangle::i_overlay::i_shape::int::count::PointsCount;
use i_triangle::i_overlay::i_shape::int::shape::{IntShape, IntShapes};

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
        Self {
            start: value.start as u64,
            end: value.end as u64,
        }
    }
}

impl From<RangeFFI> for Range<usize> {
    #[inline]
    fn from(value: RangeFFI) -> Self {
        (value.start as usize)..(value.end as usize)
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
    #[inline]
    pub fn with_capacity(points: usize, contours: usize, shapes: usize) -> Self {
        let mut buffer = Self::default();
        buffer.reserve(points, contours, shapes);
        buffer
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.flat_points.is_empty()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.flat_points.clear();
        self.contour_ranges.clear();
        self.shape_ranges.clear();
    }

    #[inline]
    pub fn set_shapes(&mut self, shapes: &[IntShape<i32>]) {
        let point_count = shapes.points_count();
        let contour_count: usize = shapes.iter().map(IntShape::len).sum();
        let shape_count = shapes.len();

        let mut core =
            CoreFlatShapesBuffer::<i32>::with_capacity(point_count, contour_count, shape_count);
        core.set_with_shapes(shapes);
        self.set_from_core(&core);
    }

    #[inline]
    pub fn push_shapes(&mut self, shapes: &[IntShape<i32>]) {
        if shapes.is_empty() {
            return;
        }

        if self.is_empty() {
            self.set_shapes(shapes);
            return;
        }

        let mut core = self.to_core();
        for shape in shapes {
            core.add_shape(shape);
        }

        self.set_from_core(&core);
    }

    #[inline]
    pub fn to_shapes(&self) -> IntShapes<i32> {
        self.to_core().to_shapes()
    }

    #[inline]
    fn set_from_core(&mut self, core: &CoreFlatShapesBuffer<i32>) {
        self.clear_and_reserve(
            core.points.len().saturating_mul(2),
            core.contour_ranges.len(),
            core.shape_ranges.len(),
        );

        for point in &core.points {
            self.flat_points.push(point.x);
            self.flat_points.push(point.y);
        }

        self.contour_ranges.extend(
            core.contour_ranges
                .iter()
                .map(|r| (r.start.saturating_mul(2))..(r.end.saturating_mul(2)))
                .map(RangeFFI::from),
        );
        self.shape_ranges
            .extend(core.shape_ranges.iter().cloned().map(RangeFFI::from));
    }

    #[inline]
    fn to_core(&self) -> CoreFlatShapesBuffer<i32> {
        let mut points = Vec::with_capacity(self.flat_points.len() / 2);
        for chunk in self.flat_points.chunks_exact(2) {
            points.push(IntPoint::new(chunk[0], chunk[1]));
        }

        let contour_ranges = self
            .contour_ranges
            .iter()
            .copied()
            .map(Range::<usize>::from)
            .map(|r| (r.start / 2)..(r.end / 2))
            .collect();
        let shape_ranges = self
            .shape_ranges
            .iter()
            .copied()
            .map(Range::<usize>::from)
            .collect();

        CoreFlatShapesBuffer {
            points,
            contour_ranges,
            shape_ranges,
        }
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

impl From<&[IntShape<i32>]> for FlatShapesBuffer {
    #[inline]
    fn from(shapes: &[IntShape<i32>]) -> Self {
        let mut buffer = FlatShapesBuffer::default();
        buffer.set_shapes(shapes);
        buffer
    }
}

impl From<&IntShapes<i32>> for FlatShapesBuffer {
    #[inline]
    fn from(shapes: &IntShapes<i32>) -> Self {
        Self::from(shapes.as_slice())
    }
}
