//! Submodule providing implementations for the procedure code generation.
use std::{fmt::Display, path::Path};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};
use webcodegen::TableExtensionNetwork;

use crate::procedure_codegen::ProcedureCodegen;

#[derive(Default, Debug, Clone)]
/// Builder for the `ProcedureCodegen`.
pub struct ProcedureCodegenBuilder<'a> {
    /// Whether to generate the procedure impls.
    generate_procedure_impls: bool,
    /// Whether to generate the procedure template impls.
    generate_procedure_template_impls: bool,
    /// Whether to generate the procedure initializer impls.
    generate_procedure_initializer_impls: bool,
    /// Whether to beautify the generated code.
    beautify: bool,
    extension_network: Option<&'a TableExtensionNetwork>,
    /// The directory where to output the generated code.
    output_directory: Option<&'a Path>,
}

impl<'a> ProcedureCodegenBuilder<'a> {
    #[must_use]
    /// Sets whether to generate the procedure impls.
    pub fn generate_procedure_impls(mut self) -> Self {
        self.generate_procedure_impls = true;
        self
    }

    #[must_use]
    /// Sets whether to generate the procedure template impls.
    pub fn generate_procedure_template_impls(mut self) -> Self {
        self.generate_procedure_template_impls = true;
        self
    }

    #[must_use]
    /// Sets whether to generate the procedure initializer impls.
    pub fn generate_procedure_initializer_impls(mut self) -> Self {
        self.generate_procedure_initializer_impls = true;
        self
    }

    #[must_use]
    /// Sets whether to beautify the generated code.
    pub fn beautify(mut self) -> Self {
        self.beautify = true;
        self
    }

    #[must_use]
    /// Sets the extension network codegen.
    pub fn extension_network(mut self, network: &'a TableExtensionNetwork) -> Self {
        self.extension_network = Some(network);
        self
    }

    #[must_use]
    /// Sets the output directory for the generated code.
    pub fn output_directory(mut self, directory: &'a Path) -> Self {
        self.output_directory = Some(directory);
        self
    }
}

#[derive(Debug, Clone)]
/// Attributes for the `ProcedureCodegenBuilder`.
pub enum ProcedureCodegenAttribute {
    GenerateEnum,
    GenerateProcedureImpls,
    GenerateProcedureTemplateImpls,
    GenerateProcedureInitializerImpls,
    ExtensionNetwork,
    Beautify,
    OutputDirectory,
}

impl Display for ProcedureCodegenAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcedureCodegenAttribute::GenerateEnum => write!(f, "generate_enum"),
            ProcedureCodegenAttribute::GenerateProcedureImpls => {
                write!(f, "generate_procedure_impls")
            }
            ProcedureCodegenAttribute::GenerateProcedureTemplateImpls => {
                write!(f, "generate_procedure_template_impls")
            }
            ProcedureCodegenAttribute::GenerateProcedureInitializerImpls => {
                write!(f, "generate_procedure_initializer_impls")
            }
            ProcedureCodegenAttribute::ExtensionNetwork => write!(f, "extension_network"),
            ProcedureCodegenAttribute::Beautify => write!(f, "beautify"),
            ProcedureCodegenAttribute::OutputDirectory => write!(f, "output_directory"),
        }
    }
}

#[derive(Debug)]
/// Errors that can occur during the building of a `ProcedureCodegen`.
pub enum ProcedureCodegenBuilderError {
    /// Error related to the builder itself.
    Builder(BuilderError<ProcedureCodegenAttribute>),
}

impl Display for ProcedureCodegenBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcedureCodegenBuilderError::Builder(err) => write!(f, "Builder error: {err}"),
        }
    }
}

impl From<BuilderError<ProcedureCodegenAttribute>> for ProcedureCodegenBuilderError {
    fn from(error: BuilderError<ProcedureCodegenAttribute>) -> Self {
        ProcedureCodegenBuilderError::Builder(error)
    }
}

impl core::error::Error for ProcedureCodegenBuilderError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            ProcedureCodegenBuilderError::Builder(err) => Some(err),
        }
    }
}

impl IsCompleteBuilder for ProcedureCodegenBuilder<'_> {
    fn is_complete(&self) -> bool {
        self.output_directory.is_some() && self.extension_network.is_some()
    }
}

impl Attributed for ProcedureCodegenBuilder<'_> {
    type Attribute = ProcedureCodegenAttribute;
}

impl<'a> Builder for ProcedureCodegenBuilder<'a> {
    type Error = ProcedureCodegenBuilderError;
    type Object = ProcedureCodegen<'a>;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(ProcedureCodegen {
            generate_procedure_impls: self.generate_procedure_impls,
            generate_procedure_template_impls: self.generate_procedure_template_impls,
            beautify: self.beautify,
            extension_network: self.extension_network.ok_or(BuilderError::IncompleteBuild(
                ProcedureCodegenAttribute::ExtensionNetwork,
            ))?,
            output_directory: self
                .output_directory
                .ok_or(BuilderError::IncompleteBuild(ProcedureCodegenAttribute::OutputDirectory))?,
        })
    }
}
