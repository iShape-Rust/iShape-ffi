use crate::bool::{IntFillRule, IntOverlayOptions};
use i_triangle::int::validation::Validation as CoreValidation;

/// FFI-safe representation of `i_triangle::int::validation::Validation`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntTriangulatorValidation {
    pub fill_rule: IntFillRule,
    pub options: IntOverlayOptions,
}

impl From<IntTriangulatorValidation> for CoreValidation {
    #[inline]
    fn from(value: IntTriangulatorValidation) -> Self {
        Self {
            fill_rule: value.fill_rule.into(),
            options: value.options.into(),
        }
    }
}

impl From<CoreValidation> for IntTriangulatorValidation {
    #[inline]
    fn from(value: CoreValidation) -> Self {
        Self {
            fill_rule: value.fill_rule.into(),
            options: value.options.into(),
        }
    }
}

impl Default for IntTriangulatorValidation {
    #[inline]
    fn default() -> Self {
        use i_triangle::i_overlay::core::overlay::IntOverlayOptions as CoreOverlayOptions;

        Self {
            fill_rule: IntFillRule::NonZero,
            options: CoreOverlayOptions::keep_output_points().into(),
        }
    }
}
