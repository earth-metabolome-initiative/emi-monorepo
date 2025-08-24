//! Submodule providing

mod builder;
mod enum_codegen;
mod procedure_impls;
mod procedure_initializer_impls;
mod procedure_model_impls;
use std::{fs::OpenOptions, io::Write, path::Path};

pub use builder::ProcedureCodegenBuilder;
use prettyplease::unparse;
use proc_macro2::TokenStream;
use quote::quote;
use syn::File;
use time_requirements::{prelude::TimeTracker, task::Task};
use webcodegen::codegen::{CODEGEN_DIRECTORY, CODEGEN_STRUCTS_MODULE, CODEGEN_TRAITS_MODULE};

use crate::errors::Error;

/// Constant defining the module name for procedure enum codegen.
pub const PROCEDURE_ENUM_MODULE: &str = "procedure_enum";
/// Constant defining the module name for procedure trait implementations.
pub const PROCEDURE_TRAIT_IMPL_MODULE: &str = "procedures";
/// Constant defining the module name for procedure model trait implementations.
pub const PROCEDURE_MODEL_TRAIT_IMPL_MODULE: &str = "procedure_models";

#[derive(Debug, Clone)]
/// Main struct for the procedure code generation.
pub struct ProcedureCodegen<'a> {
    /// Whether to generate the enum codegen.
    generate_enum: bool,
    /// Whether to generate the procedure impls.
    generate_procedure_impls: bool,
    /// Whether to generate the procedure model impls.
    generate_procedure_model_impls: bool,
    /// Whether to generate the procedure initializer impls.
    generate_procedure_initializer_impls: bool,
    /// Whether to beautify the generated code.
    beautify: bool,
    /// The directory where to output the generated code.
    output_directory: &'a Path,
}

impl<'a> ProcedureCodegen<'a> {
    /// Returns a new `ProcedureCodegenBuilder`.
    pub fn builder() -> ProcedureCodegenBuilder<'a> {
        ProcedureCodegenBuilder::default()
    }

    /// Dispatches beautification for the provided `TokenStream`, if requested.
    pub(crate) fn beautify_code(&self, code: &TokenStream) -> Result<String, Error> {
        if !self.beautify {
            return Ok(code.to_string());
        }

        let code_string = code.to_string();

        // Parse the generated code string into a syn::Item
        let syntax_tree: File = syn::parse_str(&code_string)?;

        // Use prettyplease to format the syntax tree
        let formatted_code = unparse(&syntax_tree);

        Ok(formatted_code)
    }

    /// Generates the code for the procedure codegen.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    /// * `table_catalog` - The name of the database catalog (database name).
    ///
    /// # Errors
    ///
    /// * Returns an error if the code generation fails.
    pub fn generate(
        &self,
        conn: &mut diesel::PgConnection,
        table_catalog: &str,
    ) -> Result<TimeTracker, crate::errors::Error> {
        let mut time_tracker = TimeTracker::new("Procedure Codegen");
        let codegen_directory = self.output_directory.join(CODEGEN_DIRECTORY);
        let traits_directory = codegen_directory.join(CODEGEN_TRAITS_MODULE);
        let traits_module = traits_directory.with_extension("rs");
        let structs_directory = codegen_directory.join(CODEGEN_STRUCTS_MODULE);
        let structs_module = structs_directory.with_extension("rs");
        let mut extended_traits_module = TokenStream::new();
        let mut extended_structs_module = TokenStream::new();

        if self.generate_enum {
            let task = Task::new("Enum Codegen");
            self.enum_codegen(
                structs_directory.join(PROCEDURE_ENUM_MODULE).as_path(),
                table_catalog,
                conn,
            )?;
            let module_ident =
                syn::Ident::new(PROCEDURE_ENUM_MODULE, proc_macro2::Span::call_site());
            extended_structs_module.extend(quote! {
            mod #module_ident;
            });
            time_tracker.add_completed_task(task);
        }
        if self.generate_procedure_impls {
            let task = Task::new("Procedure Impl Codegen");
            self.procedure_impls(
                traits_directory.join(PROCEDURE_TRAIT_IMPL_MODULE).as_path(),
                table_catalog,
                conn,
            )?;
            let module_ident =
                syn::Ident::new(PROCEDURE_TRAIT_IMPL_MODULE, proc_macro2::Span::call_site());
            extended_traits_module.extend(quote::quote! {
                mod #module_ident;
            });
            time_tracker.add_completed_task(task);
        }
        if self.generate_procedure_model_impls {
            let task = Task::new("Procedure Model Impl Codegen");
            self.procedure_model_impls(traits_directory.as_path(), table_catalog, conn)?;
            let module_ident =
                syn::Ident::new(PROCEDURE_MODEL_TRAIT_IMPL_MODULE, proc_macro2::Span::call_site());
            extended_traits_module.extend(quote::quote! {
                mod #module_ident;
            });
            time_tracker.add_completed_task(task);
        }
        if self.generate_procedure_initializer_impls {
            let task = Task::new("Procedure Initializer Impl Codegen");
            self.procedure_initializer_impls(traits_directory.as_path(), conn)?;
            time_tracker.add_completed_task(task);
        }

        // We extend the traits module with the generated code by appending to the file.
        writeln!(
            OpenOptions::new().append(true).open(traits_module)?,
            "{}",
            self.beautify_code(&extended_traits_module)?
        )?;

        // We extend the structs module with the generated code by appending to the
        // file.
        writeln!(
            OpenOptions::new().append(true).open(structs_module)?,
            "{}",
            self.beautify_code(&extended_structs_module)?
        )?;

        Ok(time_tracker)
    }
}
