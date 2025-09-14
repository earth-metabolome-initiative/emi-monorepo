//! Submodule providing a module to generate the foreign procedure template
//! class.

use mermaid::prelude::{StyleClassBuilder, StyleProperty};

pub(super) const FOREIGN_PROCEDURE_TEMPLATE_CLASS_NAME: &str = "foreign_procedure_template";

pub(super) fn foreign_procedure_template_class() -> StyleClassBuilder {
    StyleClassBuilder::default()
        .name(FOREIGN_PROCEDURE_TEMPLATE_CLASS_NAME)
        .unwrap()
        .property(StyleProperty::StrokeDasharray(5, 5))
        .unwrap()
}
