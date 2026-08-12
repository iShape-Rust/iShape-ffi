use alloc::vec::Vec;
use core::slice;

use i_triangle::i_overlay::i_float::float::point::FloatPoint;
use i_triangle::i_overlay::i_shape::flat::float::FloatFlatContoursBuffer;
use i_triangle::i_overlay::mesh::outline::offset::OutlineOffset;
use i_triangle::i_overlay::mesh::stroke::offset::StrokeOffset;
use i_triangle::i_overlay::mesh::style::{LineCap, LineJoin, OutlineStyle, StrokeStyle};
use i_triangle::i_overlay::mesh::variable_stroke::offset::VariableStrokeOffset;
use i_triangle::i_overlay::mesh::variable_stroke::{StrokeVertex, VariableStrokeStyle};

use crate::{FlatF64ShapesBuffer, FloatFlatShapeHierarchy, RangeFFI};

type VariableStrokeContour = Vec<StrokeVertex<FloatPoint<f64>>>;
type PreparedVariableStroke = (VariableStrokeContour, VariableStrokeStyle<f64>);

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

    if count == 0 || !count.is_multiple_of(2) || points.is_null() {
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

    let Some((contour, style)) = decode_variable_stroke(
        vertices,
        count,
        is_closed_path,
        join_kind,
        join_value,
        start_cap_kind,
        start_cap_value,
        end_cap_kind,
        end_cap_value,
    ) else {
        return false;
    };

    let shapes = contour.variable_stroke(style);

    let buffer = unsafe { &mut *output };
    buffer.set_shapes(&shapes);

    true
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_variable_stroke_f64_contour_to_flat_hierarchy_styled(
    vertices: *const f64,
    count: usize,
    is_closed_path: bool,
    join_kind: u32,
    join_value: f64,
    start_cap_kind: u32,
    start_cap_value: f64,
    end_cap_kind: u32,
    end_cap_value: f64,
    output: *mut FloatFlatShapeHierarchy,
) -> bool {
    if output.is_null() {
        return false;
    }

    let Some((contour, style)) = decode_variable_stroke(
        vertices,
        count,
        is_closed_path,
        join_kind,
        join_value,
        start_cap_kind,
        start_cap_value,
        end_cap_kind,
        end_cap_value,
    ) else {
        return false;
    };

    let hierarchy = contour.variable_stroke_hierarchy(style);
    let buffer = unsafe { &mut *output };
    buffer.set_from_core(&hierarchy);

    true
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_variable_stroke_f64_contours_to_flat_hierarchy_styled(
    vertices: *const f64,
    count: usize,
    contour_ranges: *const RangeFFI,
    contour_count: usize,
    is_closed_path: bool,
    join_kind: u32,
    join_value: f64,
    start_cap_kind: u32,
    start_cap_value: f64,
    end_cap_kind: u32,
    end_cap_value: f64,
    output: *mut FloatFlatShapeHierarchy,
) -> bool {
    if output.is_null()
        || vertices.is_null()
        || count == 0
        || !count.is_multiple_of(3)
        || contour_ranges.is_null()
        || contour_count == 0
    {
        return false;
    }

    let Some(style) = decode_variable_stroke_style(
        join_kind,
        join_value,
        start_cap_kind,
        start_cap_value,
        end_cap_kind,
        end_cap_value,
    ) else {
        return false;
    };

    let vertices_slice = unsafe { slice::from_raw_parts(vertices, count) };
    let ranges = unsafe { slice::from_raw_parts(contour_ranges, contour_count) };
    let mut contours = Vec::with_capacity(contour_count);
    for range in ranges {
        let start = range.start as usize;
        let end = range.end as usize;
        if start >= end || end > count || !start.is_multiple_of(3) || !end.is_multiple_of(3) {
            return false;
        }

        let Some(contour) =
            decode_variable_stroke_contour(&vertices_slice[start..end], is_closed_path)
        else {
            return false;
        };
        contours.push(contour);
    }

    let hierarchy = contours.variable_stroke_hierarchy(style);
    let buffer = unsafe { &mut *output };
    buffer.set_from_core(&hierarchy);

    true
}

#[allow(clippy::too_many_arguments)]
fn decode_variable_stroke(
    vertices: *const f64,
    count: usize,
    is_closed_path: bool,
    join_kind: u32,
    join_value: f64,
    start_cap_kind: u32,
    start_cap_value: f64,
    end_cap_kind: u32,
    end_cap_value: f64,
) -> Option<PreparedVariableStroke> {
    if count == 0 || !count.is_multiple_of(3) || vertices.is_null() {
        return None;
    }

    let vertices_slice = unsafe { slice::from_raw_parts(vertices, count) };
    let contour = decode_variable_stroke_contour(vertices_slice, is_closed_path)?;
    let style = decode_variable_stroke_style(
        join_kind,
        join_value,
        start_cap_kind,
        start_cap_value,
        end_cap_kind,
        end_cap_value,
    )?;
    Some((contour, style))
}

fn decode_variable_stroke_contour(
    vertices: &[f64],
    is_closed_path: bool,
) -> Option<VariableStrokeContour> {
    let vertex_count = vertices.len() / 3;
    if vertex_count < 2 {
        return None;
    }

    let mut contour: Vec<StrokeVertex<FloatPoint<f64>>> = Vec::with_capacity(vertex_count);
    for chunk in vertices.chunks_exact(3) {
        let width = chunk[2];
        if !width.is_finite() || width <= 0.0 {
            return None;
        }

        contour.push(StrokeVertex::new(
            FloatPoint::new(chunk[0], chunk[1]),
            width,
        ));
    }

    // iOverlay 8 represents closure in the centerline itself instead of taking
    // a separate flag. Keep the existing FFI contract by closing the path here.
    if is_closed_path {
        let first = contour[0];
        let last = contour[contour.len() - 1];
        if first.point.x != last.point.x || first.point.y != last.point.y {
            contour.push(first);
        }
    }

    Some(contour)
}

fn decode_variable_stroke_style(
    join_kind: u32,
    join_value: f64,
    start_cap_kind: u32,
    start_cap_value: f64,
    end_cap_kind: u32,
    end_cap_value: f64,
) -> Option<VariableStrokeStyle<f64>> {
    let join = decode_line_join(join_kind, join_value)?;
    let start_cap = decode_line_cap(start_cap_kind, start_cap_value)?;
    let end_cap = decode_line_cap(end_cap_kind, end_cap_value)?;

    // iOverlay's variable-width stroke currently uses round joins and caps.
    // Preserve the existing FFI signature and use the first round-style angle
    // supplied by callers as the shared tessellation angle.
    let round_angle = match (join, start_cap, end_cap) {
        (LineJoin::Round(value), _, _) => value,
        (_, LineCap::Round(value), _) => value,
        (_, _, LineCap::Round(value)) => value,
        _ => 0.1,
    };
    Some(VariableStrokeStyle::new().round_angle(round_angle))
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f64::consts::PI;

    fn closed_square(min: f64, max: f64, width: f64) -> Vec<StrokeVertex<FloatPoint<f64>>> {
        [(min, min), (max, min), (max, max), (min, max), (min, min)]
            .map(|(x, y)| StrokeVertex::new(FloatPoint::new(x, y), width))
            .to_vec()
    }

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

    #[test]
    fn variable_stroke_f64_contour_to_flat_styled_closes_path() {
        let vertices = [0.0, 0.0, 2.0, 10.0, 0.0, 4.0, 5.0, 10.0, 3.0];
        let mut output = FlatF64ShapesBuffer::default();

        let ok = ishape_variable_stroke_f64_contour_to_flat_styled(
            vertices.as_ptr(),
            vertices.len(),
            true,
            2,
            0.25 * PI,
            1,
            0.25 * PI,
            1,
            0.25 * PI,
            &mut output,
        );

        assert!(ok);
        assert!(!output.is_empty());
    }

    #[test]
    fn float_flat_shape_hierarchy_preserves_nested_links() {
        let paths = vec![
            closed_square(0.0, 100.0, 10.0),
            closed_square(30.0, 70.0, 10.0),
        ];
        let mut vertices = Vec::new();
        let mut ranges = Vec::new();
        for path in paths {
            let start = vertices.len();
            for vertex in path {
                vertices.extend([vertex.point.x, vertex.point.y, vertex.width]);
            }
            ranges.push(RangeFFI {
                start: start as u64,
                end: vertices.len() as u64,
            });
        }

        let mut output = FloatFlatShapeHierarchy::default();
        let ok = ishape_variable_stroke_f64_contours_to_flat_hierarchy_styled(
            vertices.as_ptr(),
            vertices.len(),
            ranges.as_ptr(),
            ranges.len(),
            true,
            2,
            0.1,
            1,
            0.1,
            1,
            0.1,
            &mut output,
        );

        assert!(ok);
        assert_eq!(output.shapes.shape_ranges.len(), 2);
        assert_eq!(output.links.len(), 1);

        let link = output.links[0];
        assert_ne!(link.parent_shape_index, link.child_shape_index);
        assert!(link.parent_shape_index < output.shapes.shape_ranges.len());
        assert!(link.child_shape_index < output.shapes.shape_ranges.len());
        let parent_contours = output.shapes.shape_ranges[link.parent_shape_index];
        assert!(
            (parent_contours.start as usize..parent_contours.end as usize)
                .contains(&link.parent_contour_index)
        );
    }
}
