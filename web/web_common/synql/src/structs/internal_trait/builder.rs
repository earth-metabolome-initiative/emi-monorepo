//! Submodule defining a builder for the `TraitDef` struct.

use std::{error::Error, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};
use quote::ToTokens;

use crate::structs::{
    Documentation, TraitDef, Publicness, TraitVariantRef, WhereClause, internal_struct::Method,
};

#[derive(Default)]
/// Builder for the `TraitDef` struct.
pub struct TraitDefBuilder {
    /// Name of the trait.
    name: Option<String>,
    /// Optional path for external traits.
    path: Option<syn::Path>,
    /// Publicness of the trait.
    publicness: Option<Publicness>,
    /// Internal token streams defined within the trait.
    methods: Vec<Method>,
    /// Trait documentation.
    documentation: Option<Documentation>,
    /// Where clauses for the trait.
    where_clauses: Vec<WhereClause>,
    /// Generics for the trait.
    generics: Vec<syn::GenericParam>,
    /// Generic default values.
    generic_defaults: Vec<Option<crate::structs::DataVariantRef>>,
    /// Super traits for the trait.
    super_traits: Vec<TraitVariantRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `TraitDef` struct.
pub enum TraitDefAttribute {
    /// Name of the trait.
    Name,
    /// Publicness of the trait.
    Publicness,
    /// Internal token streams defined within the trait.
    InternalTokens,
    /// Trait documentation.
    Documentation,
    /// Where statements for the trait.
    WhereClauses,
    /// Generics for the trait.
    Generics,
    /// Super traits for the trait.
    SuperTraits,
}

impl Display for TraitDefAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TraitDefAttribute::Name => write!(f, "name"),
            TraitDefAttribute::Publicness => write!(f, "publicness"),
            TraitDefAttribute::InternalTokens => write!(f, "methods"),
            TraitDefAttribute::Documentation => write!(f, "documentation"),
            TraitDefAttribute::WhereClauses => write!(f, "where_clauses"),
            TraitDefAttribute::Generics => write!(f, "generics"),
            TraitDefAttribute::SuperTraits => write!(f, "super_traits"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of an
/// `TraitDef`.
pub enum TraitDefBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<TraitDefAttribute>),
    /// The name of the trait is invalid.
    InvalidName,
    /// Duplicate method names found.
    DuplicateMethodName(String),
    /// Duplicate where clause found.
    DuplicateWhereClause(String),
    /// Duplicate generic found.
    DuplicateGeneric(String),
}

impl Display for TraitDefBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TraitDefBuilderError::Builder(e) => write!(f, "Builder error: {e}"),
            TraitDefBuilderError::InvalidName => write!(f, "Invalid trait name"),
            TraitDefBuilderError::DuplicateMethodName(name) => {
                write!(f, "Duplicate method name found in trait: {name}")
            }
            TraitDefBuilderError::DuplicateWhereClause(clause) => {
                write!(f, "Duplicate where clause found in trait: {clause}")
            }
            TraitDefBuilderError::DuplicateGeneric(generic) => {
                write!(f, "Duplicate generic found in trait: {generic}")
            }
        }
    }
}

impl Error for TraitDefBuilderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TraitDefBuilderError::Builder(e) => Some(e),
            _ => None,
        }
    }
}

impl TraitDefBuilder {
    /// Sets the name of the trait.
    ///
    /// # Arguments
    /// * `name` - The name of the trait.
    pub fn name<S: ToString + ?Sized>(mut self, name: &S) -> Result<Self, TraitDefBuilderError> {
        let name = name.to_string();
        if name.trim().is_empty()
            || name.contains(' ')
            || !name.chars().all(|c| c.is_alphanumeric() || c == '_')
        {
            return Err(TraitDefBuilderError::InvalidName);
        }
        self.name = Some(name);
        Ok(self)
    }

    /// Sets the path of the trait (for external traits).
    ///
    /// # Arguments
    /// * `path` - The path to the trait in the external crate.
    pub fn path(mut self, path: syn::Path) -> Self {
        self.path = Some(path);
        self
    }

    /// Sets the publicness of the trait.
    ///
    /// # Arguments
    /// * `publicness` - The publicness of the trait.
    pub fn publicness(mut self, publicness: Publicness) -> Self {
        self.publicness = Some(publicness);
        self
    }

    /// Sets the trait as public.
    pub fn public(mut self) -> Self {
        self.publicness = Some(Publicness::Public);
        self
    }

    /// Sets the trait as private.
    pub fn private(mut self) -> Self {
        self.publicness = Some(Publicness::Private);
        self
    }

    /// Sets the documentation of the trait.
    ///
    /// # Arguments
    /// * `documentation` - The documentation of the trait.
    pub fn documentation(mut self, documentation: Documentation) -> Self {
        self.documentation = Some(documentation);
        self
    }

    /// Adds a method to the trait.
    ///
    /// # Arguments
    /// * `method` - The method to add.
    pub fn method(mut self, method: Method) -> Result<Self, TraitDefBuilderError> {
        if self.methods.iter().any(|m| m.name() == method.name()) {
            return Err(TraitDefBuilderError::DuplicateMethodName(method.name().to_string()));
        }
        self.methods.push(method);
        Ok(self)
    }

    /// Adds several methods to the trait.
    ///
    /// # Arguments
    /// * `methods` - The methods to add.
    pub fn methods<I>(mut self, methods: I) -> Result<Self, TraitDefBuilderError>
    where
        I: IntoIterator<Item = Method>,
    {
        for method in methods {
            self = self.method(method)?;
        }
        Ok(self)
    }

    /// Adds a generic to the trait.
    ///
    /// # Arguments
    /// * `generic` - The generic to add.
    pub fn generic(mut self, generic: syn::GenericParam) -> Self {
        if !self.generics.contains(&generic) {
            self.generics.push(generic);
        }
        self
    }

    /// Adds several generics to the trait.
    ///
    /// # Arguments
    /// * `generics` - The generics to add.
    pub fn generics<I>(mut self, generics: I) -> Self
    where
        I: IntoIterator<Item = syn::GenericParam>,
    {
        for generic in generics {
            self = self.generic(generic);
        }
        self
    }

    /// Adds a generic with a default value.
    ///
    /// # Arguments
    /// * `generic` - The generic parameter.
    /// * `default` - The default value for the generic.
    pub fn generic_with_default(
        mut self,
        generic: syn::GenericParam,
        default: Option<crate::structs::DataVariantRef>,
    ) -> Self {
        if !self.generics.contains(&generic) {
            self.generics.push(generic);
            self.generic_defaults.push(default);
        }
        self
    }

    /// Adds a where clause to the trait.
    ///
    /// # Arguments
    /// * `where_clause` - The where clause to add.
    pub fn where_clause(
        mut self,
        where_clause: WhereClause,
    ) -> Result<Self, TraitDefBuilderError> {
        if self.where_clauses.contains(&where_clause) {
            return Err(TraitDefBuilderError::DuplicateWhereClause(
                where_clause.to_token_stream().to_string(),
            ));
        }
        self.where_clauses.push(where_clause);
        Ok(self)
    }

    /// Adds several where clauses to the trait.
    ///
    /// # Arguments
    /// * `where_clauses` - The where clauses to add.
    pub fn where_clauses<I>(mut self, where_clauses: I) -> Result<Self, TraitDefBuilderError>
    where
        I: IntoIterator<Item = WhereClause>,
    {
        for where_clause in where_clauses {
            self = self.where_clause(where_clause)?;
        }
        Ok(self)
    }

    /// Adds a super trait to the trait.
    ///
    /// # Arguments
    /// * `super_trait` - The super trait to add.
    pub fn super_trait(mut self, super_trait: TraitVariantRef) -> Self {
        if !self.super_traits.contains(&super_trait) {
            self.super_traits.push(super_trait);
        }
        self
    }

    /// Adds several super traits to the trait.
    ///
    /// # Arguments
    /// * `super_traits` - The super traits to add.
    pub fn super_traits<I>(mut self, super_traits: I) -> Self
    where
        I: IntoIterator<Item = TraitVariantRef>,
    {
        for super_trait in super_traits {
            self = self.super_trait(super_trait);
        }
        self
    }

    /// Adds the `Sized` super-trait to the trait.
    pub fn sized(self) -> Self {
        self.super_trait(TraitVariantRef::sized())
    }
}

impl Attributed for TraitDefBuilder {
    type Attribute = TraitDefAttribute;
}

impl IsCompleteBuilder for TraitDefBuilder {
    fn is_complete(&self) -> bool {
        self.name.is_some() && self.publicness.is_some() && self.documentation.is_some()
    }
}

impl Builder for TraitDefBuilder {
    type Error = BuilderError<TraitDefAttribute>;
    type Object = TraitDef;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(TraitDef {
            name: self.name.ok_or(BuilderError::IncompleteBuild(TraitDefAttribute::Name))?,
            path: self.path,
            publicness: self
                .publicness
                .ok_or(BuilderError::IncompleteBuild(TraitDefAttribute::Publicness))?,
            methods: self.methods,
            documentation: self
                .documentation
                .ok_or(BuilderError::IncompleteBuild(TraitDefAttribute::Documentation))?,
            where_clauses: self.where_clauses,
            generics: self.generics,
            generic_defaults: self.generic_defaults,
            super_traits: self.super_traits,
        })
    }
}
