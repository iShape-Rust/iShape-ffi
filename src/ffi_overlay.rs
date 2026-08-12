use alloc::vec::Vec;
use core::slice;
use std::boxed::Box;

use i_triangle::i_overlay::core::solver::Solver;
use i_triangle::i_overlay::float::overlay::FloatOverlay as CoreFloatOverlay;

use crate::{
    FlatF64ShapesBuffer, FlatShapesBuffer, Float64Overlay, Float64OverlayOptions,
    FloatFlatShapeHierarchy, IntFillRule, IntOverlay, IntOverlayOptions, IntOverlayRule,
    IntShapeType,
};

#[unsafe(no_mangle)]
pub extern "C" fn ishape_overlay_int_create(
    capacity: usize,
    options: IntOverlayOptions,
) -> *mut IntOverlay {
    Box::into_raw(Box::new(IntOverlay::new(capacity, options)))
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_overlay_int_free(handle: *mut IntOverlay) {
    if handle.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(handle));
    }
}

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
        unsafe { slice::from_raw_parts(points, count) }
    };

    let overlay = unsafe { &mut *handle };
    overlay.add_contour(points_slice, shape_type.into()).is_ok()
}

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

#[unsafe(no_mangle)]
pub extern "C" fn ishape_overlay_f64_create(
    capacity: usize,
    options: Float64OverlayOptions,
) -> *mut Float64Overlay {
    Box::into_raw(Box::new(Float64Overlay::new(capacity, options)))
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_overlay_f64_free(handle: *mut Float64Overlay) {
    if handle.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(handle));
    }
}

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

#[unsafe(no_mangle)]
pub extern "C" fn ishape_overlay_f64_overlay_into_flat_hierarchy(
    handle: *mut Float64Overlay,
    overlay_rule: IntOverlayRule,
    fill_rule: IntFillRule,
    output: *mut FloatFlatShapeHierarchy,
) -> bool {
    if handle.is_null() || output.is_null() {
        return false;
    }

    let overlay = unsafe { &*handle };
    let hierarchy = overlay.overlay_hierarchy(overlay_rule.into(), fill_rule.into());
    let buffer = unsafe { &mut *output };
    buffer.set_from_core(&hierarchy);

    true
}

#[unsafe(no_mangle)]
pub extern "C" fn ishape_overlay_f64_flat_shapes_into_flat(
    subject: *const FlatF64ShapesBuffer,
    clip: *const FlatF64ShapesBuffer,
    overlay_rule: IntOverlayRule,
    fill_rule: IntFillRule,
    options: Float64OverlayOptions,
    output: *mut FlatF64ShapesBuffer,
) -> bool {
    if subject.is_null() || clip.is_null() || output.is_null() {
        return false;
    }

    let subject_shapes = unsafe { &*subject }.to_shapes();
    let clip_shapes = unsafe { &*clip }.to_shapes();

    let subject_contours: Vec<_> = subject_shapes.into_iter().flatten().collect();
    let clip_contours: Vec<_> = clip_shapes.into_iter().flatten().collect();

    let solver = Solver {
        multithreading: None,
        ..Solver::default()
    };

    let mut overlay = CoreFloatOverlay::with_subj_and_clip_custom(
        &subject_contours,
        &clip_contours,
        options.into(),
        solver,
    );

    let shapes = overlay.overlay(overlay_rule.into(), fill_rule.into());
    let buffer = unsafe { &mut *output };
    buffer.set_shapes(&shapes);

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_overlay_hierarchy_exports_nested_links() {
        let mut overlay = Float64Overlay::new(3, Float64OverlayOptions::default());
        let contours = [
            [0.0, 0.0, 100.0, 0.0, 100.0, 100.0, 0.0, 100.0],
            [10.0, 10.0, 10.0, 90.0, 90.0, 90.0, 90.0, 10.0],
            [20.0, 20.0, 80.0, 20.0, 80.0, 80.0, 20.0, 80.0],
        ];
        for contour in contours {
            overlay
                .add_contour(&contour, IntShapeType::Subject.into())
                .unwrap();
        }

        let mut output = FloatFlatShapeHierarchy::default();
        let ok = ishape_overlay_f64_overlay_into_flat_hierarchy(
            &mut overlay,
            IntOverlayRule::Subject,
            IntFillRule::EvenOdd,
            &mut output,
        );

        assert!(ok);
        assert_eq!(output.shapes.shape_ranges.len(), 2);
        assert_eq!(output.links.len(), 1);

        let link = output.links[0];
        assert!(link.parent_shape_index < output.shapes.shape_ranges.len());
        assert!(link.child_shape_index < output.shapes.shape_ranges.len());
        let parent_contours = output.shapes.shape_ranges[link.parent_shape_index];
        assert!(
            (parent_contours.start as usize..parent_contours.end as usize)
                .contains(&link.parent_contour_index)
        );
    }
}
