//! Submodule defining supporting structs for the procedure code generation.

pub(crate) mod procedure_templates;
pub(crate) mod procedures;
pub use procedure_templates::PROCEDURE_TEMPLATES_SCHEMA;
pub(crate) use procedure_templates::ProcedureTemplate;
pub use procedures::PROCEDURES_SCHEMA;
pub(crate) use procedures::Procedure;
