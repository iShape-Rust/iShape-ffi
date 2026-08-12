use alloc::vec::Vec;

use i_triangle::i_overlay::core::hierarchy::ChildLink;
use i_triangle::i_overlay::float::hierarchy::FloatFlatShapeHierarchy as CoreFloatFlatShapeHierarchy;
use i_triangle::i_overlay::i_float::float::point::FloatPoint;

use super::FlatF64ShapesBuffer;

/// ABI-stable hierarchy link exposed to C and Swift.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShapeHierarchyLinkFFI {
    pub parent_shape_index: usize,
    pub parent_contour_index: usize,
    pub child_shape_index: usize,
}

impl From<ChildLink> for ShapeHierarchyLinkFFI {
    #[inline]
    fn from(link: ChildLink) -> Self {
        Self {
            parent_shape_index: link.parent_shape_index,
            parent_contour_index: link.parent_contour_index,
            child_shape_index: link.child_shape_index,
        }
    }
}

/// Flattened f64 shapes and their immediate nesting relationships.
#[derive(Debug, Clone, Default)]
pub struct FloatFlatShapeHierarchy {
    pub shapes: FlatF64ShapesBuffer,
    pub links: Vec<ShapeHierarchyLinkFFI>,
}

impl FloatFlatShapeHierarchy {
    #[inline]
    pub fn clear(&mut self) {
        self.shapes.clear();
        self.links.clear();
    }

    #[inline]
    pub(crate) fn set_from_core(
        &mut self,
        hierarchy: &CoreFloatFlatShapeHierarchy<FloatPoint<f64>>,
    ) {
        self.shapes.set_from_core(&hierarchy.shapes);
        self.links.clear();
        self.links.extend(
            hierarchy
                .links
                .iter()
                .copied()
                .map(ShapeHierarchyLinkFFI::from),
        );
    }
}
