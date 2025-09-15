//! Submodule providing a module to generate the default procedure template
//! class.

use mermaid::prelude::{Color, StyleClassBuilder, StyleProperty};

pub(super) fn procedure_fill_color() -> Color {
    Color::from((236, 237, 238))
}

pub(super) fn procedure_stroke_color() -> Color {
    Color::from((213, 212, 211))
}

pub(super) const PROCEDURE_ARROW_CLASS_NAME: &str = "procedure_arrow";

pub(super) fn procedure_arrow_class() -> StyleClassBuilder {
    StyleClassBuilder::default()
        .name(PROCEDURE_ARROW_CLASS_NAME)
        .unwrap()
        .property(StyleProperty::Stroke(Color::from((26, 26, 26))))
        .unwrap()
}
