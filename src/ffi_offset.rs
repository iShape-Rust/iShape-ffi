use alloc::vec::Vec;
use core::slice;

use i_triangle::i_overlay::i_float::float::point::FloatPoint;
use i_triangle::i_overlay::i_shape::flat::float::FloatFlatContoursBuffer;
use i_triangle::i_overlay::mesh::outline::offset::OutlineOffset;
use i_triangle::i_overlay::mesh::stroke::offset::StrokeOffset;
use i_triangle::i_overlay::mesh::style::{LineCap, LineJoin, OutlineStyle, StrokeStyle};
use i_triangle::i_overlay::mesh::variable_stroke::offset::VariableStrokeOffset;
use i_triangle::i_overlay::mesh::variable_stroke::{StrokeVertex, VariableStrokeStyle};

use crate::FlatF64ShapesBuffer;

#[unsafe(no_mangle)]
pub extern "C" fn ishape_outline_f64_flat_shapes_to_flat(
    input: *const FlatF64ShapesBuffer,
    offset: f64,
    output: *mut FlatF64ShapesBuffer,
) -> bool {
    if input.is_null() || output.is_null() {
        return false;
    }

    let input = unsafe { &*input };

    let buffer = unsafe { &mut *output };

    if input.is_empty() {
        buffer.clear();
        return true;
    }

    let style = OutlineStyle::new(offset);
    let outlined_shapes = input.outline(&style);

    buffer.set_shapes(&outlined_shapes);

    true
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_outline_f64_flat_contours_to_flat(
    input: *const FlatF64ShapesBuffer,
    offset: f64,
    output: *mut FlatF64ShapesBuffer,
) -> bool {
    if input.is_null() || output.is_null() {
        return false;
    }

    let input = unsafe { &*input };
    let buffer = unsafe { &mut *output };

    if input.is_empty() {
        buffer.clear();
        return true;
    }

    let style = OutlineStyle::new(offset);
    let mut contours_buffer = FloatFlatContoursBuffer::<[f64; 2]>::default();
    input.outline_into(&style, &mut contours_buffer);

    let contours = contours_buffer.to_contours();

    buffer.set_shapes(&[contours]);

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
fn decode_line_cap(kind: u32, value: f64) -> Option<LineCap<FloatPoint<f64>>> {
    match kind {
        0 => Some(LineCap::Butt),
        1 => value.is_finite().then_some(LineCap::Round(value)),
        2 => Some(LineCap::Square),
        _ => None,
    }
}

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

#[unsafe(no_mangle)]
pub extern "C" fn ishape_variable_stroke_f64_contour_to_flat_styled(
    vertices: *const f64,
    count: usize,
    is_closed_path: bool,
    join_kind: u32,
    join_value: f64,
    start_cap_kind: u32,
    start_cap_value: f64,
    end_cap_kind: u32,
    end_cap_value: f64,
    output: *mut FlatF64ShapesBuffer,
) -> bool {
    if output.is_null() {
        return false;
    }

    if count == 0 || count % 3 != 0 || vertices.is_null() {
        return false;
    }

    let vertices_slice = unsafe { slice::from_raw_parts(vertices, count) };
    let vertex_count = vertices_slice.len() / 3;
    if vertex_count < 2 {
        return false;
    }

    let mut contour: Vec<StrokeVertex<FloatPoint<f64>>> = Vec::with_capacity(vertex_count);
    for chunk in vertices_slice.chunks_exact(3) {
        let width = chunk[2];
        if !width.is_finite() || width <= 0.0 {
            return false;
        }

        contour.push(StrokeVertex::new(
            FloatPoint::new(chunk[0], chunk[1]),
            width,
        ));
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

    let style = VariableStrokeStyle::new()
        .line_join(join)
        .start_cap(start_cap)
        .end_cap(end_cap);
    let shapes = contour.variable_stroke(style, is_closed_path);

    let buffer = unsafe { &mut *output };
    buffer.set_shapes(&shapes);

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f64::consts::PI;

    #[test]
    fn variable_stroke_f64_contour_to_flat_styled_outputs_shapes() {
        let vertices = [0.0, 0.0, 2.0, 10.0, 0.0, 8.0, 20.0, 10.0, 4.0];
        let mut output = FlatF64ShapesBuffer::default();

        let ok = ishape_variable_stroke_f64_contour_to_flat_styled(
            vertices.as_ptr(),
            vertices.len(),
            false,
            2,
            0.25 * PI,
            0,
            0.0,
            2,
            0.0,
            &mut output,
        );

        assert!(ok);
        assert!(!output.is_empty());
    }
}
