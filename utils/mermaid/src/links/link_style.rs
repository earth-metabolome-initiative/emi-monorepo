//! Submodule providing a struct grouping the aesthetic properties of a link
//! style.

use crate::links::{arrow_shape::ArrowShape, line_style::LineStyle};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinkStyle {
    /// The number of segments composing the link style.
    length: u8,
    /// The line style of the link.
    line_style: LineStyle,
    /// The left arrow shape of the link, if any.
    left_arrow_shape: Option<ArrowShape>,
    /// The right arrow shape of the link, if any.
    right_arrow_shape: Option<ArrowShape>,
}
