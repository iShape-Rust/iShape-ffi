use core::{ptr, slice};
use std::boxed::Box;

use crate::{FlatF64ShapesBuffer, FlatShapesBuffer, RangeFFI};

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_shapes_create() -> *mut FlatShapesBuffer {
    Box::into_raw(Box::new(FlatShapesBuffer::default()))
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_shapes_with_capacity(
    points: usize,
    contours: usize,
    shapes: usize,
) -> *mut FlatShapesBuffer {
    Box::into_raw(Box::new(FlatShapesBuffer::with_capacity(
        points, contours, shapes,
    )))
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_shapes_clear(buffer: *mut FlatShapesBuffer) {
    if let Some(buffer) = unsafe { buffer.as_mut() } {
        buffer.clear();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_shapes_free(buffer: *mut FlatShapesBuffer) {
    if buffer.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(buffer));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_shapes_points_ptr(buffer: *const FlatShapesBuffer) -> *const i32 {
    unsafe {
        buffer
            .as_ref()
            .map_or(ptr::null(), |buffer| buffer.flat_points.as_ptr())
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_shapes_points_len(buffer: *const FlatShapesBuffer) -> usize {
    unsafe { buffer.as_ref().map_or(0, |buffer| buffer.flat_points.len()) }
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_shapes_contours_ptr(
    buffer: *const FlatShapesBuffer,
) -> *const RangeFFI {
    unsafe {
        buffer
            .as_ref()
            .map_or(ptr::null(), |buffer| buffer.contour_ranges.as_ptr())
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_shapes_contours_len(buffer: *const FlatShapesBuffer) -> usize {
    unsafe {
        buffer
            .as_ref()
            .map_or(0, |buffer| buffer.contour_ranges.len())
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_shapes_shapes_ptr(
    buffer: *const FlatShapesBuffer,
) -> *const RangeFFI {
    unsafe {
        buffer
            .as_ref()
            .map_or(ptr::null(), |buffer| buffer.shape_ranges.as_ptr())
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_shapes_shapes_len(buffer: *const FlatShapesBuffer) -> usize {
    unsafe {
        buffer
            .as_ref()
            .map_or(0, |buffer| buffer.shape_ranges.len())
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f64_shapes_create() -> *mut FlatF64ShapesBuffer {
    Box::into_raw(Box::new(FlatF64ShapesBuffer::default()))
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f64_shapes_with_capacity(
    points: usize,
    contours: usize,
    shapes: usize,
) -> *mut FlatF64ShapesBuffer {
    Box::into_raw(Box::new(FlatF64ShapesBuffer::with_capacity(
        points, contours, shapes,
    )))
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f64_shapes_clear(buffer: *mut FlatF64ShapesBuffer) {
    if let Some(buffer) = unsafe { buffer.as_mut() } {
        buffer.clear();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f64_shapes_free(buffer: *mut FlatF64ShapesBuffer) {
    if buffer.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(buffer));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f64_shapes_points_ptr(
    buffer: *const FlatF64ShapesBuffer,
) -> *const f64 {
    unsafe {
        buffer
            .as_ref()
            .map_or(ptr::null(), |buffer| buffer.flat_points.as_ptr())
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f64_shapes_points_len(buffer: *const FlatF64ShapesBuffer) -> usize {
    unsafe { buffer.as_ref().map_or(0, |buffer| buffer.flat_points.len()) }
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f64_shapes_contours_ptr(
    buffer: *const FlatF64ShapesBuffer,
) -> *const RangeFFI {
    unsafe {
        buffer
            .as_ref()
            .map_or(ptr::null(), |buffer| buffer.contour_ranges.as_ptr())
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f64_shapes_contours_len(buffer: *const FlatF64ShapesBuffer) -> usize {
    unsafe {
        buffer
            .as_ref()
            .map_or(0, |buffer| buffer.contour_ranges.len())
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f64_shapes_shapes_ptr(
    buffer: *const FlatF64ShapesBuffer,
) -> *const RangeFFI {
    unsafe {
        buffer
            .as_ref()
            .map_or(ptr::null(), |buffer| buffer.shape_ranges.as_ptr())
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f64_shapes_shapes_len(buffer: *const FlatF64ShapesBuffer) -> usize {
    unsafe {
        buffer
            .as_ref()
            .map_or(0, |buffer| buffer.shape_ranges.len())
    }
}

#[inline]
fn validate_flat_f64_shape_layout(
    point_len: usize,
    contours: &[RangeFFI],
    shapes: &[RangeFFI],
) -> bool {
    if point_len % 2 != 0 {
        return false;
    }

    if contours.is_empty() || shapes.is_empty() {
        return point_len == 0 && contours.is_empty() && shapes.is_empty();
    }

    for range in contours {
        let start = range.start as usize;
        let end = range.end as usize;
        if start >= end || end > point_len {
            return false;
        }
        if (end - start) % 2 != 0 {
            return false;
        }
    }

    for range in shapes {
        let start = range.start as usize;
        let end = range.end as usize;
        if start >= end || end > contours.len() {
            return false;
        }
    }

    true
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f64_shapes_set_flat(
    buffer: *mut FlatF64ShapesBuffer,
    points: *const f64,
    points_count: usize,
    contours: *const RangeFFI,
    contours_count: usize,
    shapes: *const RangeFFI,
    shapes_count: usize,
) -> bool {
    if buffer.is_null() {
        return false;
    }

    let points_slice: &[f64] = if points_count == 0 {
        &[]
    } else {
        if points.is_null() {
            return false;
        }
        unsafe { slice::from_raw_parts(points, points_count) }
    };

    let contour_slice: &[RangeFFI] = if contours_count == 0 {
        &[]
    } else {
        if contours.is_null() {
            return false;
        }
        unsafe { slice::from_raw_parts(contours, contours_count) }
    };

    let shape_slice: &[RangeFFI] = if shapes_count == 0 {
        &[]
    } else {
        if shapes.is_null() {
            return false;
        }
        unsafe { slice::from_raw_parts(shapes, shapes_count) }
    };

    if !validate_flat_f64_shape_layout(points_slice.len(), contour_slice, shape_slice) {
        return false;
    }

    let buffer = unsafe { &mut *buffer };
    buffer.clear();
    buffer.flat_points.extend_from_slice(points_slice);
    buffer.contour_ranges.extend_from_slice(contour_slice);
    buffer.shape_ranges.extend_from_slice(shape_slice);

    true
}
