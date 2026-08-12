use alloc::vec::Vec;
use core::ops::Range;
use core::slice;
use i_triangle::i_overlay::i_float::float::compatible::FloatPointCompatible;
use i_triangle::i_overlay::i_float::float::point::FloatPoint;
use i_triangle::i_overlay::i_shape::base::data::{Shape, Shapes};
use i_triangle::i_overlay::i_shape::flat::float::FloatFlatShapesBuffer as CoreFloatFlatShapesBuffer;
use i_triangle::i_overlay::i_shape::source::resource::ShapeResource;

use super::int_shapes_buffer::RangeFFI;

type Float64Point = FloatPoint<f64>;
type Float64Shape = Shape<Float64Point>;
type Float64Shapes = Shapes<Float64Point>;
type CoreFlatF64ShapesBuffer = CoreFloatFlatShapesBuffer<Float64Point>;

#[inline]
fn shape_range_ffi_to_core(range: RangeFFI) -> Range<usize> {
    (range.start as usize)..(range.end as usize)
}

#[inline]
fn contour_range_ffi_to_core(range: RangeFFI) -> Range<usize> {
    ((range.start as usize) / 2)..((range.end as usize) / 2)
}

#[inline]
fn contour_range_core_to_ffi(range: Range<usize>) -> RangeFFI {
    RangeFFI {
        start: range.start.saturating_mul(2) as u64,
        end: range.end.saturating_mul(2) as u64,
    }
}

pub struct FlatF64ShapesBufferResourceIterator<'a> {
    buffer: &'a FlatF64ShapesBuffer,
    index: usize,
}

impl<'a> FlatF64ShapesBufferResourceIterator<'a> {
    #[inline]
    fn with_buffer(buffer: &'a FlatF64ShapesBuffer) -> Self {
        Self { buffer, index: 0 }
    }
}

impl<'a> Iterator for FlatF64ShapesBufferResourceIterator<'a> {
    type Item = &'a [[f64; 2]];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let points = self.buffer.points_as_pairs()?;

        while self.index < self.buffer.contour_ranges.len() {
            let i = self.index;
            self.index += 1;

            let range = self.buffer.contour_ranges[i];
            let start = range.start as usize;
            let end = range.end as usize;

            if start >= end || end > self.buffer.flat_points.len() {
                continue;
            }

            if !start.is_multiple_of(2) || !end.is_multiple_of(2) {
                continue;
            }

            let pair_range = (start / 2)..(end / 2);
            return Some(&points[pair_range]);
        }

        None
    }
}

impl ShapeResource<[f64; 2]> for FlatF64ShapesBuffer {
    type ResourceIter<'a>
        = FlatF64ShapesBufferResourceIterator<'a>
    where
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        FlatF64ShapesBufferResourceIterator::with_buffer(self)
    }
}

/// Flattened container for `Float64Shapes` data that is easy to consume from Swift.
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct FlatF64ShapesBuffer {
    pub flat_points: Vec<f64>,
    pub contour_ranges: Vec<RangeFFI>,
    pub shape_ranges: Vec<RangeFFI>,
}

impl FlatF64ShapesBuffer {
    #[inline]
    pub fn with_capacity(points: usize, contours: usize, shapes: usize) -> Self {
        let mut buffer = FlatF64ShapesBuffer::default();
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
    pub fn set_shapes<P>(&mut self, shapes: &[Shape<P>])
    where
        P: FloatPointCompatible<Scalar = f64>,
    {
        let point_count: usize = shapes
            .iter()
            .map(|shape| shape.iter().map(Vec::len).sum::<usize>())
            .sum();
        let contour_count: usize = shapes.iter().map(Vec::len).sum();
        let shape_count = shapes.len();

        let mut core =
            CoreFlatF64ShapesBuffer::with_capacity(point_count, contour_count, shape_count);

        let mut points_offset = 0;
        let mut contours_offset = 0;
        for shape in shapes {
            let shape_start = contours_offset;
            for contour in shape {
                let len = contour.len();
                core.points
                    .extend(contour.iter().map(|p| FloatPoint::new(p.x(), p.y())));
                core.contour_ranges.push(points_offset..points_offset + len);
                points_offset += len;
                contours_offset += 1;
            }
            core.shape_ranges.push(shape_start..contours_offset);
        }

        self.set_from_core(&core);
    }

    #[inline]
    pub(crate) fn set_contours_as_shapes<P>(&mut self, contours: &[Vec<P>])
    where
        P: FloatPointCompatible<Scalar = f64>,
    {
        let point_count = contours.iter().map(Vec::len).sum();
        let mut core =
            CoreFlatF64ShapesBuffer::with_capacity(point_count, contours.len(), contours.len());

        for contour in contours {
            let point_start = core.points.len();
            core.points
                .extend(contour.iter().map(|p| FloatPoint::new(p.x(), p.y())));
            core.contour_ranges.push(point_start..core.points.len());
            let contour_index = core.contour_ranges.len() - 1;
            core.shape_ranges.push(contour_index..contour_index + 1);
        }

        self.set_from_core(&core);
    }

    #[inline]
    pub fn push_shapes<P>(&mut self, shapes: &[Shape<P>])
    where
        P: FloatPointCompatible<Scalar = f64>,
    {
        if shapes.is_empty() {
            return;
        }

        if self.is_empty() {
            self.set_shapes(shapes);
            return;
        }

        let mut core = self.to_core();
        let mut points_offset = core.points.len();
        let mut contours_offset = core.contour_ranges.len();

        for shape in shapes {
            let shape_start = contours_offset;
            for contour in shape {
                let len = contour.len();
                core.points
                    .extend(contour.iter().map(|p| FloatPoint::new(p.x(), p.y())));
                core.contour_ranges.push(points_offset..points_offset + len);
                points_offset += len;
                contours_offset += 1;
            }
            core.shape_ranges.push(shape_start..contours_offset);
        }

        self.set_from_core(&core);
    }

    #[inline]
    pub fn to_shapes(&self) -> Float64Shapes {
        self.to_core().to_shapes()
    }

    #[inline]
    fn points_as_pairs(&self) -> Option<&[[f64; 2]]> {
        if !self.flat_points.len().is_multiple_of(2) {
            return None;
        }

        let len = self.flat_points.len() / 2;
        Some(unsafe { slice::from_raw_parts(self.flat_points.as_ptr().cast::<[f64; 2]>(), len) })
    }

    #[inline]
    pub(crate) fn set_from_core(&mut self, core: &CoreFlatF64ShapesBuffer) {
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
                .cloned()
                .map(contour_range_core_to_ffi),
        );
        self.shape_ranges
            .extend(core.shape_ranges.iter().cloned().map(RangeFFI::from));
    }

    #[inline]
    fn to_core(&self) -> CoreFlatF64ShapesBuffer {
        let mut points = Vec::with_capacity(self.flat_points.len() / 2);
        for coords in self.flat_points.chunks_exact(2) {
            points.push(FloatPoint::new(coords[0], coords[1]));
        }

        let contour_ranges = self
            .contour_ranges
            .iter()
            .copied()
            .map(contour_range_ffi_to_core)
            .collect();
        let shape_ranges = self
            .shape_ranges
            .iter()
            .copied()
            .map(shape_range_ffi_to_core)
            .collect();

        CoreFlatF64ShapesBuffer {
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

impl From<&[Float64Shape]> for FlatF64ShapesBuffer {
    #[inline]
    fn from(shapes: &[Float64Shape]) -> Self {
        let mut buffer = FlatF64ShapesBuffer::default();
        buffer.set_shapes(shapes);
        buffer
    }
}

impl From<&Float64Shapes> for FlatF64ShapesBuffer {
    #[inline]
    fn from(shapes: &Float64Shapes) -> Self {
        FlatF64ShapesBuffer::from(shapes.as_slice())
    }
}
