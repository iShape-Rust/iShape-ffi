use core::{ptr, slice};

extern crate alloc;

pub mod bool;
pub mod shape;

pub use crate::bool::{
    IntContourDirection, IntFillRule, IntOverlay, IntOverlayOptions, IntOverlayRule, IntShapeType,
};
pub use crate::shape::{FlatShapesBuffer, RangeFFI};

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
