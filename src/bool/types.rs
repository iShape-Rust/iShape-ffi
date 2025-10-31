use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::{
    ContourDirection, IntOverlayOptions as CoreOverlayOptions, ShapeType,
};
use i_overlay::core::overlay_rule::OverlayRule;

/// Wrapper enum mirroring `i_overlay::core::overlay::ShapeType` for FFI consumers.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntShapeType {
    Subject = 0,
    Clip = 1,
}

impl From<IntShapeType> for ShapeType {
    #[inline]
    fn from(value: IntShapeType) -> Self {
        match value {
            IntShapeType::Subject => ShapeType::Subject,
            IntShapeType::Clip => ShapeType::Clip,
        }
    }
}

impl From<ShapeType> for IntShapeType {
    #[inline]
    fn from(value: ShapeType) -> Self {
        match value {
            ShapeType::Subject => IntShapeType::Subject,
            ShapeType::Clip => IntShapeType::Clip,
        }
    }
}

/// FFI-safe representation of `ContourDirection`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntContourDirection {
    CounterClockwise = 0,
    Clockwise = 1,
}

impl From<IntContourDirection> for ContourDirection {
    #[inline]
    fn from(value: IntContourDirection) -> Self {
        match value {
            IntContourDirection::CounterClockwise => ContourDirection::CounterClockwise,
            IntContourDirection::Clockwise => ContourDirection::Clockwise,
        }
    }
}

impl From<ContourDirection> for IntContourDirection {
    #[inline]
    fn from(value: ContourDirection) -> Self {
        match value {
            ContourDirection::CounterClockwise => IntContourDirection::CounterClockwise,
            ContourDirection::Clockwise => IntContourDirection::Clockwise,
        }
    }
}

/// FFI-safe options struct mirroring `i_overlay::core::overlay::IntOverlayOptions`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntOverlayOptions {
    pub preserve_input_collinear: bool,
    pub output_direction: IntContourDirection,
    pub preserve_output_collinear: bool,
    pub min_output_area: u64,
}

impl From<IntOverlayOptions> for CoreOverlayOptions {
    #[inline]
    fn from(value: IntOverlayOptions) -> Self {
        Self {
            preserve_input_collinear: value.preserve_input_collinear,
            output_direction: value.output_direction.into(),
            preserve_output_collinear: value.preserve_output_collinear,
            min_output_area: value.min_output_area,
        }
    }
}

impl From<CoreOverlayOptions> for IntOverlayOptions {
    #[inline]
    fn from(value: CoreOverlayOptions) -> Self {
        Self {
            preserve_input_collinear: value.preserve_input_collinear,
            output_direction: value.output_direction.into(),
            preserve_output_collinear: value.preserve_output_collinear,
            min_output_area: value.min_output_area,
        }
    }
}

impl Default for IntOverlayOptions {
    #[inline]
    fn default() -> Self {
        CoreOverlayOptions::default().into()
    }
}

/// FFI-safe enum mirroring `OverlayRule`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntOverlayRule {
    Subject = 0,
    Clip = 1,
    Intersect = 2,
    Union = 3,
    Difference = 4,
    InverseDifference = 5,
    Xor = 6,
}

impl From<IntOverlayRule> for OverlayRule {
    #[inline]
    fn from(value: IntOverlayRule) -> Self {
        match value {
            IntOverlayRule::Subject => OverlayRule::Subject,
            IntOverlayRule::Clip => OverlayRule::Clip,
            IntOverlayRule::Intersect => OverlayRule::Intersect,
            IntOverlayRule::Union => OverlayRule::Union,
            IntOverlayRule::Difference => OverlayRule::Difference,
            IntOverlayRule::InverseDifference => OverlayRule::InverseDifference,
            IntOverlayRule::Xor => OverlayRule::Xor,
        }
    }
}

impl From<OverlayRule> for IntOverlayRule {
    #[inline]
    fn from(value: OverlayRule) -> Self {
        match value {
            OverlayRule::Subject => IntOverlayRule::Subject,
            OverlayRule::Clip => IntOverlayRule::Clip,
            OverlayRule::Intersect => IntOverlayRule::Intersect,
            OverlayRule::Union => IntOverlayRule::Union,
            OverlayRule::Difference => IntOverlayRule::Difference,
            OverlayRule::InverseDifference => IntOverlayRule::InverseDifference,
            OverlayRule::Xor => IntOverlayRule::Xor,
        }
    }
}

/// FFI-safe enum mirroring `FillRule`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntFillRule {
    EvenOdd = 0,
    NonZero = 1,
    Positive = 2,
    Negative = 3,
}

impl From<IntFillRule> for FillRule {
    #[inline]
    fn from(value: IntFillRule) -> Self {
        match value {
            IntFillRule::EvenOdd => FillRule::EvenOdd,
            IntFillRule::NonZero => FillRule::NonZero,
            IntFillRule::Positive => FillRule::Positive,
            IntFillRule::Negative => FillRule::Negative,
        }
    }
}

impl From<FillRule> for IntFillRule {
    #[inline]
    fn from(value: FillRule) -> Self {
        match value {
            FillRule::EvenOdd => IntFillRule::EvenOdd,
            FillRule::NonZero => IntFillRule::NonZero,
            FillRule::Positive => IntFillRule::Positive,
            FillRule::Negative => IntFillRule::Negative,
        }
    }
}
