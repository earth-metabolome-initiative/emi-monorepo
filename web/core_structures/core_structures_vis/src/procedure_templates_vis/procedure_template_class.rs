//! Submodule providing a module to generate the default procedure template
//! class.

use mermaid::prelude::{Color, StyleClassBuilder, StyleProperty};

pub(super) fn procedure_template_class() -> StyleClassBuilder {
    StyleClassBuilder::default()
        .name("procedure_template")
        .unwrap()
        .property(StyleProperty::Fill(Color::pastel_red()))
        .unwrap()
        .property(StyleProperty::Stroke(Color::pastel_red().darken(20)))
        .unwrap()
}
