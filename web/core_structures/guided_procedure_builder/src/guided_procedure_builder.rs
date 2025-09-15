//! Submodule defining the GuidedProcedureBuilder structure and its associated
//! methods.

use std::iter::Once;

use core_structures::{ProcedureTemplate, ProcedureTemplateAssetModel};

use crate::{PTGVisitor, procedure_template_graph::ProcedureTemplateGraph};

#[derive(Debug, Clone)]
/// Struct providing functionalities to help the user concretely build a
/// procedure.
pub struct GuidedProcedureBuilder {
    /// Metadata about the procedure template being built.
    procedure_template_graph: ProcedureTemplateGraph,
}
