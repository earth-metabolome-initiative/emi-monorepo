//! Submodule providing a module to generate the default procedure template
//! class.

use mermaid::prelude::{Color, StyleClassBuilder, StyleProperty};

pub(super) const PROCEDURE_TEMPLATE_CLASS_NAME: &str = "procedure_template";

pub(super) fn procedure_template_class() -> StyleClassBuilder {
    StyleClassBuilder::default()
        .name(PROCEDURE_TEMPLATE_CLASS_NAME)
        .unwrap()
        .property(StyleProperty::Fill(Color::pastel_red()))
        .unwrap()
        .property(StyleProperty::Stroke(Color::pastel_red().darken(20)))
        .unwrap()
}
