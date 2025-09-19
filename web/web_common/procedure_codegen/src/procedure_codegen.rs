//! Submodule providing

mod builder;
mod procedure_impls;
mod procedure_template_impls;
use std::{fs::OpenOptions, io::Write, path::Path};

pub use builder::ProcedureCodegenBuilder;
use prettyplease::unparse;
use proc_macro2::TokenStream;
use syn::File;
use time_requirements::{prelude::TimeTracker, task::Task};
use webcodegen::{
    TableExtensionNetwork,
    codegen::{CODEGEN_DIRECTORY, CODEGEN_TRAITS_MODULE},
};

use crate::errors::Error;

/// Constant defining the module name for procedure trait implementations.
pub const PROCEDURE_TRAIT_IMPL_MODULE: &str = "procedures";
/// Constant defining the module name for procedure template trait
/// implementations.
pub const PROCEDURE_TEMPLATE_TRAIT_IMPL_MODULE: &str = "procedure_templates";

#[derive(Debug, Clone)]
/// Main struct for the procedure code generation.
pub struct ProcedureCodegen<'a> {
    /// Whether to generate the procedure impls.
    generate_procedure_impls: bool,
    /// Whether to generate the procedure template impls.
    generate_procedure_template_impls: bool,
    /// Whether to beautify the generated code.
    beautify: bool,
    /// The extension network codegen.
    extension_network: &'a TableExtensionNetwork,
    /// The directory where to output the generated code.
    output_directory: &'a Path,
}

impl<'a> ProcedureCodegen<'a> {
    /// Returns a new `ProcedureCodegenBuilder`.
    #[must_use]
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
    /// * `conn` - A mutable reference to a `PostgreSQL` connection.
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
        let mut extended_traits_module = TokenStream::new();

        if self.generate_procedure_impls {
            let task = Task::new("Procedure Impl Codegen");
            // We create the procedure impls subdirectory.
            let subdirectory = traits_directory.join(PROCEDURE_TRAIT_IMPL_MODULE);
            std::fs::create_dir_all(&subdirectory)?;

            self.procedure_impls(subdirectory.as_path(), table_catalog, conn)?;
            let module_ident =
                syn::Ident::new(PROCEDURE_TRAIT_IMPL_MODULE, proc_macro2::Span::call_site());
            extended_traits_module.extend(quote::quote! {
                mod #module_ident;
            });
            time_tracker.add_completed_task(task);
        }
        if self.generate_procedure_template_impls {
            let task = Task::new("procedure template Impl Codegen");
            // We create the procedure template impls subdirectory.
            let subdirectory = traits_directory.join(PROCEDURE_TEMPLATE_TRAIT_IMPL_MODULE);
            std::fs::create_dir_all(&subdirectory)?;

            self.procedure_template_impls(subdirectory.as_path(), table_catalog, conn)?;
            let module_ident = syn::Ident::new(
                PROCEDURE_TEMPLATE_TRAIT_IMPL_MODULE,
                proc_macro2::Span::call_site(),
            );
            extended_traits_module.extend(quote::quote! {
                mod #module_ident;
            });
            time_tracker.add_completed_task(task);
        }

        // We extend the traits module with the generated code by appending to the file.
        writeln!(
            OpenOptions::new().append(true).open(traits_module)?,
            "{}",
            self.beautify_code(&extended_traits_module)?
        )?;

        Ok(time_tracker)
    }
}
