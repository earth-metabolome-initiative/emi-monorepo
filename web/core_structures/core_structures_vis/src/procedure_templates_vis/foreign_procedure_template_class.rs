//! Submodule providing a module to generate the foreign procedure template
//! class.

use mermaid::prelude::{StyleClassBuilder, StyleProperty};

pub(super) fn foreign_procedure_template_class() -> StyleClassBuilder {
    StyleClassBuilder::default()
        .name("foreign_procedure_template")
        .unwrap()
        .property(StyleProperty::StrokeDasharray(5, 5))
        .unwrap()
}
