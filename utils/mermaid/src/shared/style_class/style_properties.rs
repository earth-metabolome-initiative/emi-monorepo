//! Enumeration of style properties which may be applied to nodes in a Mermaid
//! diagram.

use std::fmt::Display;

use crate::shared::style_class::{
    color::Color, font_style::FontStyle, font_weight::FontWeight, units::Unit,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `StyleProperty` enumerates all supported style properties for Mermaid class
/// definitions.
pub enum StyleProperty {
    /// Sets the fill color of the node (e.g., `fill: #ff0000`)
    Fill(Color),
    /// Sets the stroke (border) color of the node (e.g., `stroke: #000`)
    Stroke(Color),
    /// Sets the text color inside the node (e.g., `color: #333`)
    Color(Color),
    /// Sets the width of the node border (e.g., `stroke-width: 2px`)
    StrokeWidth(Unit),
    /// Sets the font size for node text (e.g., `font-size: 16px`)
    FontSize(Unit),
    /// Sets the font weight for node text (e.g., `font-weight: bold`)
    FontWeight(FontWeight),
    /// Sets the font style for node text (e.g., `font-style: italic`)
    FontStyle(FontStyle),
    /// Sets the dash pattern for the border (e.g., `stroke-dasharray: 5, 2`)
    StrokeDasharray(u8, u8),
    /// Sets the dash offset for the border (e.g., `stroke-dashoffset: 4`)
    StrokeDashoffset(u16),
    /// Sets the opacity of the node, with a value between 0 and 100,
    /// which is then normalized to a float between 0.0 and 1.0.
    Opacity(u8),
    /// Border radius for rounded corners (e.g., `border-radius: 5px`)
    BorderRadius(Unit),
}

impl StyleProperty {
    #[must_use]
    /// Returns whether the provided style property is of the same type as
    /// `self`.
    pub fn is_same_type(self, other: StyleProperty) -> bool {
        matches!(
            (self, other),
            (StyleProperty::Fill(_), StyleProperty::Fill(_))
                | (StyleProperty::Stroke(_), StyleProperty::Stroke(_))
                | (StyleProperty::Color(_), StyleProperty::Color(_))
                | (StyleProperty::StrokeWidth(_), StyleProperty::StrokeWidth(_))
                | (StyleProperty::FontSize(_), StyleProperty::FontSize(_))
                | (StyleProperty::FontWeight(_), StyleProperty::FontWeight(_))
                | (StyleProperty::FontStyle(_), StyleProperty::FontStyle(_))
                | (StyleProperty::StrokeDasharray(_, _), StyleProperty::StrokeDasharray(_, _))
                | (StyleProperty::StrokeDashoffset(_), StyleProperty::StrokeDashoffset(_))
                | (StyleProperty::Opacity(_), StyleProperty::Opacity(_))
                | (StyleProperty::BorderRadius(_), StyleProperty::BorderRadius(_))
        )
    }
}

impl Display for StyleProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StyleProperty::Fill(color) => write!(f, "fill: {}", color.to_hex()),
            StyleProperty::Stroke(color) => write!(f, "stroke: {}", color.to_hex()),
            StyleProperty::Color(color) => write!(f, "color: {}", color.to_hex()),
            StyleProperty::StrokeWidth(unit) => write!(f, "stroke-width: {unit}"),
            StyleProperty::FontSize(unit) => write!(f, "font-size: {unit}"),
            StyleProperty::FontWeight(weight) => write!(f, "font-weight: {weight}"),
            StyleProperty::FontStyle(style) => write!(f, "font-style: {style}"),
            StyleProperty::StrokeDasharray(length, gap) => {
                write!(f, "stroke-dasharray: {length}, {gap}")
            }
            StyleProperty::StrokeDashoffset(offset) => write!(f, "stroke-dashoffset: {offset}",),
            StyleProperty::Opacity(value) => write!(f, "opacity: {:.2}", f32::from(*value) / 100.0),
            StyleProperty::BorderRadius(radius) => write!(f, "rx: {radius}, ry: {radius}"),
        }
    }
}
