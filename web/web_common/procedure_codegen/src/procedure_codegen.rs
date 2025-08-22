//! Submodule providing

mod builder;
mod enum_codegen;
mod procedure_impls;
mod procedure_initializer_impls;
mod procedure_model_impls;

pub use builder::ProcedureCodegenBuilder;

#[derive(Default, Debug, Clone)]
/// Main struct for the procedure code generation.
pub struct ProcedureCodegen {
    /// Whether to generate the enum codegen.
    generate_enum: bool,
    /// Whether to generate the procedure impls.
    generate_procedure_impls: bool,
    /// Whether to generate the procedure model impls.
    generate_procedure_model_impls: bool,
    /// Whether to generate the procedure initializer impls.
    generate_procedure_initializer_impls: bool,
}

impl ProcedureCodegen {
    /// Returns a new `ProcedureCodegenBuilder`.
    pub fn builder() -> ProcedureCodegenBuilder {
        ProcedureCodegenBuilder::default()
    }

    /// Generates the code for the procedure codegen.
    pub fn generate(&self, conn: &mut diesel::PgConnection) -> Result<(), crate::errors::Error> {
        if self.generate_enum {
            self.enum_codegen(conn)?;
        }
        if self.generate_procedure_impls {
            self.procedure_impls(conn)?;
        }
        if self.generate_procedure_model_impls {
            self.procedure_model_impls(conn)?;
        }
        if self.generate_procedure_initializer_impls {
            self.procedure_initializer_impls(conn)?;
        }
        Ok(())
    }
}
