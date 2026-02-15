extern crate alloc;

use alloc::vec::Vec;
use std::boxed::Box;
use core::{ptr, slice};
use i_triangle::i_overlay::i_float::float::point::FloatPoint;
use i_triangle::i_overlay::mesh::outline::offset::OutlineOffset;
use i_triangle::i_overlay::mesh::style::{LineCap, LineJoin, OutlineStyle};
use i_triangle::i_overlay::mesh::stroke::offset::StrokeOffset;
use i_triangle::i_overlay::mesh::style::StrokeStyle;

pub mod bool;
pub mod shape;
pub mod triangle;

pub use crate::bool::{
    Float64Overlay, Float64OverlayOptions, IntContourDirection, IntFillRule, IntOverlay,
    IntOverlayOptions, IntOverlayRule, IntShapeType,
};
pub use crate::shape::{FlatF32ShapesBuffer, FlatF64ShapesBuffer, FlatShapesBuffer, RangeFFI};
pub use crate::triangle::{
    FlatF32Triangulation, FlatF64Triangulation, FlatIntTriangulation, Float32Triangulator,
    Float64Triangulator, IntTriangulationIndex, IntTriangulator, IntTriangulatorValidation,
};

/// Allocates an empty flat integer triangulation buffer.
///
/// # Safety
/// The returned pointer must be released with [`ishape_triangle_flat_triangulation_free`].
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_triangulation_create() -> *mut FlatIntTriangulation {
    Box::into_raw(Box::new(FlatIntTriangulation::default()))
}

/// Allocates a flat triangulation buffer reserving capacity for `points` vertices and `triangles`.
///
/// # Safety
/// The returned pointer must be released with [`ishape_triangle_flat_triangulation_free`].
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_triangulation_with_capacity(
    points: usize,
    triangles: usize,
) -> *mut FlatIntTriangulation {
    Box::into_raw(Box::new(FlatIntTriangulation::with_capacity(
        points, triangles,
    )))
}

/// Clears the buffer contents without releasing the allocation.
///
/// # Safety
/// Passing a null pointer is a no-op. Otherwise the pointer must be valid and uniquely owned.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_triangulation_clear(buffer: *mut FlatIntTriangulation) {
    if let Some(buffer) = unsafe { buffer.as_mut() } {
        buffer.clear();
    }
}

/// Releases a flat triangulation buffer previously allocated with
/// `ishape_triangle_flat_triangulation_*`.
///
/// # Safety
/// Passing a null pointer is a no-op. Otherwise the pointer must have been allocated by this crate.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_triangulation_free(buffer: *mut FlatIntTriangulation) {
    if buffer.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(buffer));
    }
}

/// Returns a pointer to the flattened point buffer.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_triangulation_points_ptr(
    buffer: *const FlatIntTriangulation,
) -> *const i32 {
    unsafe {
        buffer
            .as_ref()
            .map_or(ptr::null(), |buffer| buffer.flat_points.as_ptr())
    }
}

/// Returns the flattened point buffer length.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_triangulation_points_len(
    buffer: *const FlatIntTriangulation,
) -> usize {
    unsafe { buffer.as_ref().map_or(0, |buffer| buffer.flat_points.len()) }
}

/// Returns a pointer to the triangle indices buffer.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_triangulation_indices_ptr(
    buffer: *const FlatIntTriangulation,
) -> *const IntTriangulationIndex {
    unsafe {
        buffer
            .as_ref()
            .map_or(ptr::null(), |buffer| buffer.indices.as_ptr())
    }
}

/// Returns the triangle index count.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_triangulation_indices_len(
    buffer: *const FlatIntTriangulation,
) -> usize {
    unsafe { buffer.as_ref().map_or(0, |buffer| buffer.indices.len()) }
}

/// Allocates an empty flat `f32` triangulation buffer.
///
/// # Safety
/// The returned pointer must be released with [`ishape_triangle_flat_f32_triangulation_free`].
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f32_triangulation_create() -> *mut FlatF32Triangulation {
    Box::into_raw(Box::new(FlatF32Triangulation::default()))
}

/// Allocates a flat `f32` triangulation buffer reserving capacity for `points` and `triangles`.
///
/// # Safety
/// The returned pointer must be released with [`ishape_triangle_flat_f32_triangulation_free`].
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f32_triangulation_with_capacity(
    points: usize,
    triangles: usize,
) -> *mut FlatF32Triangulation {
    Box::into_raw(Box::new(FlatF32Triangulation::with_capacity(
        points, triangles,
    )))
}

/// Clears the contents of a flat `f32` triangulation buffer while keeping its allocation.
///
/// # Safety
/// Passing a null pointer is a no-op.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f32_triangulation_clear(buffer: *mut FlatF32Triangulation) {
    if let Some(buffer) = unsafe { buffer.as_mut() } {
        buffer.clear();
    }
}

/// Releases a flat `f32` triangulation buffer previously allocated with
/// `ishape_triangle_flat_f32_triangulation_*`.
///
/// # Safety
/// Passing a null pointer is a no-op.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f32_triangulation_free(buffer: *mut FlatF32Triangulation) {
    if buffer.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(buffer));
    }
}

/// Returns a pointer to the flattened single-precision point buffer.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f32_triangulation_points_ptr(
    buffer: *const FlatF32Triangulation,
) -> *const f32 {
    unsafe {
        buffer
            .as_ref()
            .map_or(ptr::null(), |buffer| buffer.flat_points.as_ptr())
    }
}

/// Returns the flattened single-precision point buffer length.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f32_triangulation_points_len(
    buffer: *const FlatF32Triangulation,
) -> usize {
    unsafe { buffer.as_ref().map_or(0, |buffer| buffer.flat_points.len()) }
}

/// Returns a pointer to the triangle indices buffer.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f32_triangulation_indices_ptr(
    buffer: *const FlatF32Triangulation,
) -> *const IntTriangulationIndex {
    unsafe {
        buffer
            .as_ref()
            .map_or(ptr::null(), |buffer| buffer.indices.as_ptr())
    }
}

/// Returns the triangle index count for the single-precision buffer.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f32_triangulation_indices_len(
    buffer: *const FlatF32Triangulation,
) -> usize {
    unsafe { buffer.as_ref().map_or(0, |buffer| buffer.indices.len()) }
}

/// Allocates an empty flat `f64` triangulation buffer.
///
/// # Safety
/// The returned pointer must be released with [`ishape_triangle_flat_f64_triangulation_free`].
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f64_triangulation_create() -> *mut FlatF64Triangulation {
    Box::into_raw(Box::new(FlatF64Triangulation::default()))
}

/// Allocates a flat `f64` triangulation buffer reserving capacity for `points` and `triangles`.
///
/// # Safety
/// The returned pointer must be released with [`ishape_triangle_flat_f64_triangulation_free`].
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f64_triangulation_with_capacity(
    points: usize,
    triangles: usize,
) -> *mut FlatF64Triangulation {
    Box::into_raw(Box::new(FlatF64Triangulation::with_capacity(
        points, triangles,
    )))
}

/// Clears the contents of a flat `f64` triangulation buffer while keeping its allocation.
///
/// # Safety
/// Passing a null pointer is a no-op.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f64_triangulation_clear(buffer: *mut FlatF64Triangulation) {
    if let Some(buffer) = unsafe { buffer.as_mut() } {
        buffer.clear();
    }
}

/// Releases a flat `f64` triangulation buffer previously allocated with
/// `ishape_triangle_flat_f64_triangulation_*`.
///
/// # Safety
/// Passing a null pointer is a no-op.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f64_triangulation_free(buffer: *mut FlatF64Triangulation) {
    if buffer.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(buffer));
    }
}

/// Returns a pointer to the flattened double-precision point buffer.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f64_triangulation_points_ptr(
    buffer: *const FlatF64Triangulation,
) -> *const f64 {
    unsafe {
        buffer
            .as_ref()
            .map_or(ptr::null(), |buffer| buffer.flat_points.as_ptr())
    }
}

/// Returns the flattened double-precision point buffer length.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f64_triangulation_points_len(
    buffer: *const FlatF64Triangulation,
) -> usize {
    unsafe { buffer.as_ref().map_or(0, |buffer| buffer.flat_points.len()) }
}

/// Returns a pointer to the triangle indices buffer for `f64` results.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f64_triangulation_indices_ptr(
    buffer: *const FlatF64Triangulation,
) -> *const IntTriangulationIndex {
    unsafe {
        buffer
            .as_ref()
            .map_or(ptr::null(), |buffer| buffer.indices.as_ptr())
    }
}

/// Returns the triangle index count for the double-precision buffer.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f64_triangulation_indices_len(
    buffer: *const FlatF64Triangulation,
) -> usize {
    unsafe { buffer.as_ref().map_or(0, |buffer| buffer.indices.len()) }
}

/// Allocates an empty flat shapes buffer on the heap.
///
/// # Safety
/// The returned pointer must be released with [`ishape_flat_shapes_free`].
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_shapes_create() -> *mut FlatShapesBuffer {
    Box::into_raw(Box::new(FlatShapesBuffer::default()))
}

/// Allocates a flat shapes buffer reserving `points`, `contours`, and `shapes` capacity.
///
/// # Safety
/// The returned pointer must be released with [`ishape_flat_shapes_free`].
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

/// Clears the buffer contents without releasing the allocation.
///
/// # Safety
/// Passing a null pointer is a no-op. Otherwise, the pointer must be valid and uniquely owned.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_shapes_clear(buffer: *mut FlatShapesBuffer) {
    if let Some(buffer) = unsafe { buffer.as_mut() } {
        buffer.clear();
    }
}

/// Releases a flat shapes buffer previously allocated with `ishape_flat_shapes_*`.
///
/// # Safety
/// Passing a null pointer is a no-op. Otherwise the pointer must have been allocated by this crate.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_shapes_free(buffer: *mut FlatShapesBuffer) {
    if buffer.is_null() {
        return;
    }

    // SAFETY: Caller guarantees the pointer originated from our allocation routines.
    unsafe {
        drop(Box::from_raw(buffer));
    }
}

/// Returns a pointer to the flattened points buffer.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_shapes_points_ptr(buffer: *const FlatShapesBuffer) -> *const i32 {
    unsafe {
        buffer
            .as_ref()
            .map_or(ptr::null(), |buffer| buffer.flat_points.as_ptr())
    }
}

/// Returns the flattened points length.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_shapes_points_len(buffer: *const FlatShapesBuffer) -> usize {
    unsafe { buffer.as_ref().map_or(0, |buffer| buffer.flat_points.len()) }
}

/// Returns a pointer to the contour ranges buffer.
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

/// Returns the number of contour ranges.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_shapes_contours_len(buffer: *const FlatShapesBuffer) -> usize {
    unsafe {
        buffer
            .as_ref()
            .map_or(0, |buffer| buffer.contour_ranges.len())
    }
}

/// Returns a pointer to the shape ranges buffer.
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

/// Returns the number of shape ranges stored.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_shapes_shapes_len(buffer: *const FlatShapesBuffer) -> usize {
    unsafe {
        buffer
            .as_ref()
            .map_or(0, |buffer| buffer.shape_ranges.len())
    }
}

/// Allocates an empty flat `f32` shapes buffer on the heap.
///
/// # Safety
/// The returned pointer must be released with [`ishape_flat_f32_shapes_free`].
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f32_shapes_create() -> *mut FlatF32ShapesBuffer {
    Box::into_raw(Box::new(FlatF32ShapesBuffer::default()))
}

/// Allocates a flat `f32` shapes buffer reserving `points`, `contours`, and `shapes` capacity.
///
/// # Safety
/// The returned pointer must be released with [`ishape_flat_f32_shapes_free`].
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f32_shapes_with_capacity(
    points: usize,
    contours: usize,
    shapes: usize,
) -> *mut FlatF32ShapesBuffer {
    Box::into_raw(Box::new(FlatF32ShapesBuffer::with_capacity(
        points, contours, shapes,
    )))
}

/// Clears the buffer contents without releasing the allocation.
///
/// # Safety
/// Passing a null pointer is a no-op. Otherwise, the pointer must be valid and uniquely owned.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f32_shapes_clear(buffer: *mut FlatF32ShapesBuffer) {
    if let Some(buffer) = unsafe { buffer.as_mut() } {
        buffer.clear();
    }
}

/// Releases a flat `f32` shapes buffer previously allocated with `ishape_flat_f32_shapes_*`.
///
/// # Safety
/// Passing a null pointer is a no-op. Otherwise the pointer must have been allocated by this crate.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f32_shapes_free(buffer: *mut FlatF32ShapesBuffer) {
    if buffer.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(buffer));
    }
}

/// Returns a pointer to the flattened points buffer.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f32_shapes_points_ptr(
    buffer: *const FlatF32ShapesBuffer,
) -> *const f32 {
    unsafe {
        buffer
            .as_ref()
            .map_or(ptr::null(), |buffer| buffer.flat_points.as_ptr())
    }
}

/// Returns the flattened points length.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f32_shapes_points_len(buffer: *const FlatF32ShapesBuffer) -> usize {
    unsafe { buffer.as_ref().map_or(0, |buffer| buffer.flat_points.len()) }
}

/// Returns a pointer to the contour ranges buffer.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f32_shapes_contours_ptr(
    buffer: *const FlatF32ShapesBuffer,
) -> *const RangeFFI {
    unsafe {
        buffer
            .as_ref()
            .map_or(ptr::null(), |buffer| buffer.contour_ranges.as_ptr())
    }
}

/// Returns the number of contour ranges.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f32_shapes_contours_len(buffer: *const FlatF32ShapesBuffer) -> usize {
    unsafe {
        buffer
            .as_ref()
            .map_or(0, |buffer| buffer.contour_ranges.len())
    }
}

/// Returns a pointer to the shape ranges buffer.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f32_shapes_shapes_ptr(
    buffer: *const FlatF32ShapesBuffer,
) -> *const RangeFFI {
    unsafe {
        buffer
            .as_ref()
            .map_or(ptr::null(), |buffer| buffer.shape_ranges.as_ptr())
    }
}

/// Returns the number of shape ranges stored.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f32_shapes_shapes_len(buffer: *const FlatF32ShapesBuffer) -> usize {
    unsafe {
        buffer
            .as_ref()
            .map_or(0, |buffer| buffer.shape_ranges.len())
    }
}

/// Allocates an empty flat `f64` shapes buffer on the heap.
///
/// # Safety
/// The returned pointer must be released with [`ishape_flat_f64_shapes_free`].
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f64_shapes_create() -> *mut FlatF64ShapesBuffer {
    Box::into_raw(Box::new(FlatF64ShapesBuffer::default()))
}

/// Allocates a flat `f64` shapes buffer reserving `points`, `contours`, and `shapes` capacity.
///
/// # Safety
/// The returned pointer must be released with [`ishape_flat_f64_shapes_free`].
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

/// Clears the buffer contents without releasing the allocation.
///
/// # Safety
/// Passing a null pointer is a no-op. Otherwise, the pointer must be valid and uniquely owned.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f64_shapes_clear(buffer: *mut FlatF64ShapesBuffer) {
    if let Some(buffer) = unsafe { buffer.as_mut() } {
        buffer.clear();
    }
}

/// Releases a flat `f64` shapes buffer previously allocated with `ishape_flat_f64_shapes_*`.
///
/// # Safety
/// Passing a null pointer is a no-op. Otherwise the pointer must have been allocated by this crate.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f64_shapes_free(buffer: *mut FlatF64ShapesBuffer) {
    if buffer.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(buffer));
    }
}

/// Returns a pointer to the flattened points buffer.
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

/// Returns the flattened points length.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f64_shapes_points_len(buffer: *const FlatF64ShapesBuffer) -> usize {
    unsafe { buffer.as_ref().map_or(0, |buffer| buffer.flat_points.len()) }
}

/// Returns a pointer to the contour ranges buffer.
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

/// Returns the number of contour ranges.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f64_shapes_contours_len(buffer: *const FlatF64ShapesBuffer) -> usize {
    unsafe {
        buffer
            .as_ref()
            .map_or(0, |buffer| buffer.contour_ranges.len())
    }
}

/// Returns a pointer to the shape ranges buffer.
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

/// Returns the number of shape ranges stored.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_flat_f64_shapes_shapes_len(buffer: *const FlatF64ShapesBuffer) -> usize {
    unsafe {
        buffer
            .as_ref()
            .map_or(0, |buffer| buffer.shape_ranges.len())
    }
}

/// Creates a new integer overlay pre-allocating space for `capacity` segment endpoints.
///
/// # Safety
/// The returned pointer must eventually be released with [`ishape_overlay_int_free`] to avoid
/// leaking resources.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_overlay_int_create(
    capacity: usize,
    options: IntOverlayOptions,
) -> *mut IntOverlay {
    Box::into_raw(Box::new(IntOverlay::new(capacity, options)))
}

/// Releases an overlay previously created with [`ishape_overlay_int_create`].
///
/// Passing a null pointer is a no-op.
///
/// # Safety
/// The pointer must have been returned by [`ishape_overlay_int_create`] and not freed already.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_overlay_int_free(handle: *mut IntOverlay) {
    if handle.is_null() {
        return;
    }

    // SAFETY: The caller guarantees `handle` came from `ishape_overlay_int_create` and has not
    // been deallocated yet.
    unsafe {
        drop(Box::from_raw(handle));
    }
}

/// Appends a contour to the overlay using a flat coordinate buffer `[x0, y0, x1, y1, ...]`.
///
/// Returns `false` if the input is invalid (null handle, null coordinates with non-zero length,
/// or an odd number of coordinates). On success the contour is queued inside the overlay.
///
/// # Safety
/// - `handle` must be a valid pointer obtained from [`ishape_overlay_int_create`].
/// - `points` must either be null with `count == 0` or point to `count` consecutive `i32` values.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_overlay_int_add_contour(
    handle: *mut IntOverlay,
    points: *const i32,
    count: usize,
    shape_type: IntShapeType,
) -> bool {
    if handle.is_null() {
        return false;
    }

    let points_slice: &[i32] = if count == 0 {
        &[]
    } else {
        if points.is_null() {
            return false;
        }

        // SAFETY: The caller guarantees `points` points to `count` elements when `count > 0`.
        unsafe { slice::from_raw_parts(points, count) }
    };

    let overlay = unsafe { &mut *handle };

    overlay.add_contour(points_slice, shape_type.into()).is_ok()
}

/// Runs the configured boolean operation, writing results into a flat buffer.
///
/// Returns `false` if any pointer is null; otherwise the output buffer is populated and `true`
/// is returned.
///
/// # Safety
/// All pointers must be valid and uniquely owned for the duration of the call.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_overlay_int_overlay_into_flat(
    handle: *mut IntOverlay,
    overlay_rule: IntOverlayRule,
    fill_rule: IntFillRule,
    output: *mut FlatShapesBuffer,
) -> bool {
    if handle.is_null() || output.is_null() {
        return false;
    }

    let overlay = unsafe { &mut *handle };
    let buffer = unsafe { &mut *output };

    let shapes = overlay.overlay(overlay_rule.into(), fill_rule.into());

    buffer.set_shapes(&shapes);

    true
}

/// Creates a new `f64` overlay pre-allocating space for contour metadata.
///
/// # Safety
/// The returned pointer must eventually be released with [`ishape_overlay_f64_free`] to avoid
/// leaking resources.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_overlay_f64_create(
    capacity: usize,
    options: Float64OverlayOptions,
) -> *mut Float64Overlay {
    Box::into_raw(Box::new(Float64Overlay::new(capacity, options)))
}

/// Releases an overlay previously created with [`ishape_overlay_f64_create`].
///
/// Passing a null pointer is a no-op.
///
/// # Safety
/// The pointer must have been returned by [`ishape_overlay_f64_create`] and not freed already.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_overlay_f64_free(handle: *mut Float64Overlay) {
    if handle.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(handle));
    }
}

/// Appends a contour to the overlay using a flat coordinate buffer `[x0, y0, x1, y1, ...]`.
///
/// Returns `false` if the input is invalid (null handle, null coordinates with non-zero length,
/// or an odd number of coordinates). On success the contour is queued inside the overlay.
///
/// # Safety
/// - `handle` must be a valid pointer obtained from [`ishape_overlay_f64_create`].
/// - `points` must either be null with `count == 0` or point to `count` consecutive `f64` values.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_overlay_f64_add_contour(
    handle: *mut Float64Overlay,
    points: *const f64,
    count: usize,
    shape_type: IntShapeType,
) -> bool {
    if handle.is_null() {
        return false;
    }

    let points_slice: &[f64] = if count == 0 {
        &[]
    } else {
        if points.is_null() {
            return false;
        }

        unsafe { slice::from_raw_parts(points, count) }
    };

    let overlay = unsafe { &mut *handle };

    overlay.add_contour(points_slice, shape_type.into()).is_ok()
}

/// Runs the configured boolean operation, writing results into a flat `f64` shapes buffer.
///
/// Returns `false` if any pointer is null; otherwise the output buffer is populated and `true`
/// is returned.
///
/// # Safety
/// All pointers must be valid and uniquely owned for the duration of the call.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_overlay_f64_overlay_into_flat(
    handle: *mut Float64Overlay,
    overlay_rule: IntOverlayRule,
    fill_rule: IntFillRule,
    output: *mut FlatF64ShapesBuffer,
) -> bool {
    if handle.is_null() || output.is_null() {
        return false;
    }

    let overlay = unsafe { &*handle };
    let buffer = unsafe { &mut *output };

    let shapes = overlay.overlay(overlay_rule.into(), fill_rule.into());

    buffer.set_shapes(&shapes);

    true
}

/// Builds an offset/buffer result from a single contour and writes it into a flat `f64` buffer.
///
/// Input contour is represented as `[x0, y0, x1, y1, ...]`.
///
/// Returns `false` when pointers are invalid, coordinate count is odd, or fewer than 3 points are
/// provided.
///
/// # Safety
/// - `points` must either be null with `count == 0` or point to `count` consecutive `f64` values.
/// - `output` must be a valid pointer to a `FlatF64ShapesBuffer`.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_outline_f64_contour_to_flat(
    points: *const f64,
    count: usize,
    offset: f64,
    output: *mut FlatF64ShapesBuffer,
) -> bool {
    if output.is_null() {
        return false;
    }

    if count == 0 || count % 2 != 0 || points.is_null() {
        return false;
    }

    let points_slice = unsafe { slice::from_raw_parts(points, count) };
    let point_count = points_slice.len() / 2;
    if point_count < 3 {
        return false;
    }

    let mut contour: Vec<FloatPoint<f64>> = Vec::with_capacity(point_count);
    for chunk in points_slice.chunks_exact(2) {
        contour.push(FloatPoint::new(chunk[0], chunk[1]));
    }

    // Public API uses positive distance as outward buffer.
    let style = OutlineStyle::new(-offset);
    let mut shapes = contour.outline(&style);

    // Single-contour fast-path in i_overlay is sensitive to winding; retry in reverse order.
    if shapes.is_empty() {
        contour.reverse();
        shapes = contour.outline(&style);
    }

    let buffer = unsafe { &mut *output };
    buffer.set_shapes(&shapes);

    true
}

#[inline]
fn decode_line_join(kind: u32, value: f64) -> Option<LineJoin<f64>> {
    match kind {
        0 => Some(LineJoin::Bevel),
        1 => value.is_finite().then_some(LineJoin::Miter(value)),
        2 => value.is_finite().then_some(LineJoin::Round(value)),
        _ => None,
    }
}

#[inline]
fn decode_line_cap(kind: u32, value: f64) -> Option<LineCap<FloatPoint<f64>, f64>> {
    match kind {
        0 => Some(LineCap::Butt),
        1 => value.is_finite().then_some(LineCap::Round(value)),
        2 => Some(LineCap::Square),
        _ => None,
    }
}

/// Builds a stroke/buffer result from a single contour and writes it into a flat `f64` buffer,
/// using explicit line join and line cap styles.
///
/// - `join_kind`: `0=Bevel`, `1=Miter`, `2=Round`
/// - `start_cap_kind` and `end_cap_kind`: `0=Butt`, `1=Round`, `2=Square`
#[unsafe(no_mangle)]
pub extern "C" fn ishape_stroke_f64_contour_to_flat_styled(
    points: *const f64,
    count: usize,
    width: f64,
    is_closed_path: bool,
    join_kind: u32,
    join_value: f64,
    start_cap_kind: u32,
    start_cap_value: f64,
    end_cap_kind: u32,
    end_cap_value: f64,
    output: *mut FlatF64ShapesBuffer,
) -> bool {
    if output.is_null() || width <= 0.0 {
        return false;
    }

    if count == 0 || count % 2 != 0 || points.is_null() {
        return false;
    }

    let points_slice = unsafe { slice::from_raw_parts(points, count) };
    let point_count = points_slice.len() / 2;
    if point_count < 2 {
        return false;
    }

    let mut contour: Vec<FloatPoint<f64>> = Vec::with_capacity(point_count);
    for chunk in points_slice.chunks_exact(2) {
        contour.push(FloatPoint::new(chunk[0], chunk[1]));
    }

    let join = if let Some(join) = decode_line_join(join_kind, join_value) {
        join
    } else {
        return false;
    };

    let start_cap = if let Some(cap) = decode_line_cap(start_cap_kind, start_cap_value) {
        cap
    } else {
        return false;
    };

    let end_cap = if let Some(cap) = decode_line_cap(end_cap_kind, end_cap_value) {
        cap
    } else {
        return false;
    };

    let style = StrokeStyle::new(width)
        .line_join(join)
        .start_cap(start_cap)
        .end_cap(end_cap);
    let shapes = contour.stroke(style, is_closed_path);

    let buffer = unsafe { &mut *output };
    buffer.set_shapes(&shapes);

    true
}

/// Creates a new integer triangulator configured for up to `max_points_count` points.
///
/// # Safety
/// The returned pointer must be released with [`ishape_triangle_int_triangulator_free`].
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_int_triangulator_create(
    max_points_count: usize,
    validation: IntTriangulatorValidation,
) -> *mut IntTriangulator {
    Box::into_raw(Box::new(IntTriangulator::new(max_points_count, validation)))
}

/// Releases a triangulator previously created with [`ishape_triangle_int_triangulator_create`].
///
/// Passing a null pointer is a no-op.
///
/// # Safety
/// The pointer must have been returned by [`ishape_triangle_int_triangulator_create`] and not
/// freed already.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_int_triangulator_free(handle: *mut IntTriangulator) {
    if handle.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(handle));
    }
}

/// Creates a new `f32` triangulator configured for up to `max_points_count` points.
///
/// # Safety
/// The returned pointer must be released with [`ishape_triangle_f32_triangulator_free`].
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_f32_triangulator_create(
    max_points_count: usize,
    validation: IntTriangulatorValidation,
) -> *mut Float32Triangulator {
    Box::into_raw(Box::new(Float32Triangulator::new(
        max_points_count,
        validation,
    )))
}

/// Releases a triangulator previously created with [`ishape_triangle_f32_triangulator_create`].
///
/// Passing a null pointer is a no-op.
///
/// # Safety
/// The pointer must have been returned by [`ishape_triangle_f32_triangulator_create`] and not
/// freed already.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_f32_triangulator_free(handle: *mut Float32Triangulator) {
    if handle.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(handle));
    }
}

/// Runs the configured `f32` triangulator, writing the result into a flat buffer.
///
/// Returns `false` if any pointer is null; otherwise the output buffer is populated and `true`
/// is returned.
///
/// # Safety
/// All pointers must be valid and uniquely owned for the duration of the call.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_f32_triangulator_triangulate_flat(
    handle: *mut Float32Triangulator,
    shapes: *const FlatF32ShapesBuffer,
    output: *mut FlatF32Triangulation,
) -> bool {
    if handle.is_null() || shapes.is_null() || output.is_null() {
        return false;
    }

    let triangulator = unsafe { &mut *handle };
    let shapes_buffer = unsafe { &*shapes };
    let mut shapes_vec = shapes_buffer.to_shapes();

    let buffer = unsafe { &mut *output };

    if shapes_vec.is_empty() {
        buffer.clear();
        return true;
    }

    let triangulation = triangulator.inner.triangulate(&shapes_vec);

    buffer.set_triangulation(&triangulation);

    // Release memory early.
    shapes_vec.clear();

    true
}

/// Creates a new `f64` triangulator configured for up to `max_points_count` points.
///
/// # Safety
/// The returned pointer must be released with [`ishape_triangle_f64_triangulator_free`].
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_f64_triangulator_create(
    max_points_count: usize,
    validation: IntTriangulatorValidation,
) -> *mut Float64Triangulator {
    Box::into_raw(Box::new(Float64Triangulator::new(
        max_points_count,
        validation,
    )))
}

/// Releases a triangulator previously created with [`ishape_triangle_f64_triangulator_create`].
///
/// Passing a null pointer is a no-op.
///
/// # Safety
/// The pointer must have been returned by [`ishape_triangle_f64_triangulator_create`] and not
/// freed already.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_f64_triangulator_free(handle: *mut Float64Triangulator) {
    if handle.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(handle));
    }
}

/// Runs the configured `f64` triangulator, writing the result into a flat buffer.
///
/// Returns `false` if any pointer is null; otherwise the output buffer is populated and `true`
/// is returned.
///
/// # Safety
/// All pointers must be valid and uniquely owned for the duration of the call.
#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_f64_triangulator_triangulate_flat(
    handle: *mut Float64Triangulator,
    shapes: *const FlatF64ShapesBuffer,
    output: *mut FlatF64Triangulation,
) -> bool {
    if handle.is_null() || shapes.is_null() || output.is_null() {
        return false;
    }

    let triangulator = unsafe { &mut *handle };
    let shapes_buffer = unsafe { &*shapes };
    let mut shapes_vec = shapes_buffer.to_shapes();

    let buffer = unsafe { &mut *output };

    if shapes_vec.is_empty() {
        buffer.clear();
        return true;
    }

    let triangulation = triangulator.inner.triangulate(&shapes_vec);

    buffer.set_triangulation(&triangulation);

    shapes_vec.clear();

    true
}
