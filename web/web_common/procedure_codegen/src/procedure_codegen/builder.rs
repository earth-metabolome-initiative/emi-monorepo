//! Submodule providing implementations for the procedure code generation.
use crate::procedure_codegen::ProcedureCodegen;

#[derive(Default, Debug, Clone)]
/// Builder for the `ProcedureCodegen`.
pub struct ProcedureCodegenBuilder {
    /// The procedure codegen instance.
    procedure_codegen: ProcedureCodegen,
}

impl ProcedureCodegenBuilder {
    #[must_use]
    /// Sets whether to generate the enum codegen.
    pub fn generate_enum(mut self, generate: bool) -> Self {
        self.procedure_codegen.generate_enum = generate;
        self
    }

    #[must_use]
    /// Sets whether to generate the procedure impls.
    pub fn generate_procedure_impls(mut self, generate: bool) -> Self {
        self.procedure_codegen.generate_procedure_impls = generate;
        self
    }

    #[must_use]
    /// Sets whether to generate the procedure model impls.
    pub fn generate_procedure_model_impls(mut self, generate: bool) -> Self {
        self.procedure_codegen.generate_procedure_model_impls = generate;
        self
    }

    #[must_use]
    /// Sets whether to generate the procedure initializer impls.
    pub fn generate_procedure_initializer_impls(mut self, generate: bool) -> Self {
        self.procedure_codegen.generate_procedure_initializer_impls = generate;
        self
    }
}

impl From<ProcedureCodegenBuilder> for ProcedureCodegen {
    fn from(builder: ProcedureCodegenBuilder) -> Self {
        builder.procedure_codegen
    }
}
