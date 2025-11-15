//! Submodule defining the `GuidedProcedurePseudocodeBuilder` structure and its
//! associated methods.

mod builder;
mod error;
mod iterator;
mod listener;
pub use builder::GuidedProcedurePseudocodeBuilder;
use listener::GPPListener;
use web_common_traits::procedures::ProcedureTemplateRoot;

use crate::{
    procedure_template_graph::ProcedureTemplateGraph, ptg_visitor::PTGVisitor,
    structs::HierarchyLike,
};
#[derive(Debug, Clone)]
/// Struct providing functionalities to help the user concretely build a
/// procedure.
pub struct GuidedProcedurePseudocode<'graph> {
    visitor: PTGVisitor<'graph, ProcedureTemplateGraph, GPPListener<'graph>>,
}

impl<'graph> GuidedProcedurePseudocode<'graph> {
    /// Creates a new `GuidedProcedurePseudocodeBuilder` instance.
    #[must_use]
    pub fn new() -> GuidedProcedurePseudocodeBuilder<'graph> {
        GuidedProcedurePseudocodeBuilder::default()
    }

    /// Writes out the guided procedure pseudocode.
    #[must_use]
    pub fn pseudocode<E>(mut self) -> String {
        let mut output = String::new();
        let root_procedure_template = self.visitor.graph().root_procedure_template();
        let variable_name = root_procedure_template.name.to_lowercase().replace(' ', "_");

        output.push_str(
            &[
                format!("let {variable_name} = GuidedProcedure::new()"),
                "    .author(author)".to_owned(),
                "    .graph(&procedure_graph)".to_owned(),
                "    .connection(portal_conn)".to_owned(),
                "    .build()?;\n\n".to_owned(),
            ]
            .join("\n"),
        );

        output.push_str(&variable_name);

        let error_type = std::any::type_name::<E>();

        for procedure in &mut self {
            let procedure_type = procedure.procedure_type();
            output.push_str(
                &[
                    format!(
                        "\n    .and_then::<{procedure_type}, {error_type}>(|mut builder, conn| {{"
                    ),
                    format!(
                        "        todo!(\"Implement the logic for \\\"{}\\\"\");",
                        procedure.name
                    ),
                    "        Ok(builder)".to_owned(),
                    "    })?".to_owned(),
                ]
                .join("\n"),
            );
        }
        output.push_str("\n    .finish()?;\n");

        output
    }
}
