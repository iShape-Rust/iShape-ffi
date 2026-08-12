use core::ptr;
use std::boxed::Box;

use i_triangle::float::triangulatable::Triangulatable;

use crate::{
    FlatF64ShapesBuffer, FlatF64Triangulation, FlatIntTriangulation, Float64Triangulator,
    IntTriangulationIndex, IntTriangulator, IntTriangulatorValidation,
};

#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_triangulation_create() -> *mut FlatIntTriangulation {
    Box::into_raw(Box::new(FlatIntTriangulation::default()))
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_triangulation_with_capacity(
    points: usize,
    triangles: usize,
) -> *mut FlatIntTriangulation {
    Box::into_raw(Box::new(FlatIntTriangulation::with_capacity(
        points, triangles,
    )))
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_triangulation_clear(buffer: *mut FlatIntTriangulation) {
    if let Some(buffer) = unsafe { buffer.as_mut() } {
        buffer.clear();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_triangulation_free(buffer: *mut FlatIntTriangulation) {
    if buffer.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(buffer));
    }
}

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

#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_triangulation_points_len(
    buffer: *const FlatIntTriangulation,
) -> usize {
    unsafe { buffer.as_ref().map_or(0, |buffer| buffer.flat_points.len()) }
}

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

#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_triangulation_indices_len(
    buffer: *const FlatIntTriangulation,
) -> usize {
    unsafe { buffer.as_ref().map_or(0, |buffer| buffer.indices.len()) }
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f64_triangulation_create() -> *mut FlatF64Triangulation {
    Box::into_raw(Box::new(FlatF64Triangulation::default()))
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f64_triangulation_with_capacity(
    points: usize,
    triangles: usize,
) -> *mut FlatF64Triangulation {
    Box::into_raw(Box::new(FlatF64Triangulation::with_capacity(
        points, triangles,
    )))
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f64_triangulation_clear(buffer: *mut FlatF64Triangulation) {
    if let Some(buffer) = unsafe { buffer.as_mut() } {
        buffer.clear();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f64_triangulation_free(buffer: *mut FlatF64Triangulation) {
    if buffer.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(buffer));
    }
}

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

#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f64_triangulation_points_len(
    buffer: *const FlatF64Triangulation,
) -> usize {
    unsafe { buffer.as_ref().map_or(0, |buffer| buffer.flat_points.len()) }
}

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

#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_flat_f64_triangulation_indices_len(
    buffer: *const FlatF64Triangulation,
) -> usize {
    unsafe { buffer.as_ref().map_or(0, |buffer| buffer.indices.len()) }
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_int_triangulator_create(
    max_points_count: usize,
    validation: IntTriangulatorValidation,
) -> *mut IntTriangulator {
    Box::into_raw(Box::new(IntTriangulator::new(max_points_count, validation)))
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_int_triangulator_free(handle: *mut IntTriangulator) {
    if handle.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(handle));
    }
}

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

#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_f64_triangulator_free(handle: *mut Float64Triangulator) {
    if handle.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(handle));
    }
}

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

#[unsafe(no_mangle)]
pub extern "C" fn ishape_triangle_f64_shapes_to_convex_polygons(
    shapes: *const FlatF64ShapesBuffer,
    output: *mut FlatF64ShapesBuffer,
) -> bool {
    if shapes.is_null() || output.is_null() {
        return false;
    }

    let shapes = unsafe { &*shapes }.to_shapes();
    let output = unsafe { &mut *output };
    if shapes.is_empty() {
        output.clear();
        return true;
    }

    let polygons = shapes
        .as_slice()
        .triangulate()
        .into_delaunay()
        .to_convex_polygons();
    output.set_contours_as_shapes(&polygons);

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use i_triangle::i_overlay::i_float::float::point::FloatPoint;

    #[test]
    fn convex_decomposition_splits_concave_shape() {
        let shape = vec![vec![
            FloatPoint::new(0.0, 0.0),
            FloatPoint::new(6.0, 0.0),
            FloatPoint::new(6.0, 2.0),
            FloatPoint::new(2.0, 2.0),
            FloatPoint::new(2.0, 6.0),
            FloatPoint::new(0.0, 6.0),
        ]];
        let input = FlatF64ShapesBuffer::from(&vec![shape]);
        let mut output = FlatF64ShapesBuffer::default();

        let ok = ishape_triangle_f64_shapes_to_convex_polygons(&input, &mut output);

        assert!(ok);
        let polygons = output.to_shapes();
        assert!(polygons.len() > 1);
        assert!(polygons.iter().all(|shape| shape.len() == 1));
        assert!(polygons.iter().all(|shape| shape[0].len() >= 3));
    }
}
