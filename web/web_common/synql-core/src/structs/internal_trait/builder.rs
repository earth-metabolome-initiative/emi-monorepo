//! Submodule defining a builder for the `InternalTrait` struct.

use std::{error::Error, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};
use quote::ToTokens;

use crate::structs::{
    Documentation, InternalTrait, Publicness, TraitVariantRef, WhereClause, internal_struct::Method,
};

#[derive(Default)]
/// Builder for the `InternalTrait` struct.
pub struct InternalTraitBuilder {
    /// Name of the trait.
    name: Option<String>,
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
    /// Super traits for the trait.
    super_traits: Vec<TraitVariantRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `InternalTrait` struct.
pub enum InternalTraitAttribute {
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

impl Display for InternalTraitAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InternalTraitAttribute::Name => write!(f, "name"),
            InternalTraitAttribute::Publicness => write!(f, "publicness"),
            InternalTraitAttribute::InternalTokens => write!(f, "methods"),
            InternalTraitAttribute::Documentation => write!(f, "documentation"),
            InternalTraitAttribute::WhereClauses => write!(f, "where_clauses"),
            InternalTraitAttribute::Generics => write!(f, "generics"),
            InternalTraitAttribute::SuperTraits => write!(f, "super_traits"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of an
/// `InternalTrait`.
pub enum InternalTraitBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<InternalTraitAttribute>),
    /// The name of the trait is invalid.
    InvalidName,
    /// Duplicate method names found.
    DuplicateMethodName(String),
    /// Duplicate where clause found.
    DuplicateWhereClause(String),
    /// Duplicate generic found.
    DuplicateGeneric(String),
}

impl Display for InternalTraitBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InternalTraitBuilderError::Builder(e) => write!(f, "Builder error: {e}"),
            InternalTraitBuilderError::InvalidName => write!(f, "Invalid trait name"),
            InternalTraitBuilderError::DuplicateMethodName(name) => {
                write!(f, "Duplicate method name found in trait: {name}")
            }
            InternalTraitBuilderError::DuplicateWhereClause(clause) => {
                write!(f, "Duplicate where clause found in trait: {clause}")
            }
            InternalTraitBuilderError::DuplicateGeneric(generic) => {
                write!(f, "Duplicate generic found in trait: {generic}")
            }
        }
    }
}

impl Error for InternalTraitBuilderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InternalTraitBuilderError::Builder(e) => Some(e),
            _ => None,
        }
    }
}

impl InternalTraitBuilder {
    /// Sets the name of the trait.
    ///
    /// # Arguments
    /// * `name` - The name of the trait.
    pub fn name<S: ToString>(mut self, name: &S) -> Result<Self, InternalTraitBuilderError> {
        let name = name.to_string();
        if name.trim().is_empty()
            || name.contains(' ')
            || !name.chars().all(|c| c.is_alphanumeric() || c == '_')
        {
            return Err(InternalTraitBuilderError::InvalidName);
        }
        self.name = Some(name);
        Ok(self)
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
    pub fn method(mut self, method: Method) -> Result<Self, InternalTraitBuilderError> {
        if self.methods.iter().any(|m| m.name() == method.name()) {
            return Err(InternalTraitBuilderError::DuplicateMethodName(method.name().to_string()));
        }
        self.methods.push(method);
        Ok(self)
    }

    /// Adds several methods to the trait.
    ///
    /// # Arguments
    /// * `methods` - The methods to add.
    pub fn methods<I>(mut self, methods: I) -> Result<Self, InternalTraitBuilderError>
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

    /// Adds a where clause to the trait.
    ///
    /// # Arguments
    /// * `where_clause` - The where clause to add.
    pub fn where_clause(
        mut self,
        where_clause: WhereClause,
    ) -> Result<Self, InternalTraitBuilderError> {
        if self.where_clauses.contains(&where_clause) {
            return Err(InternalTraitBuilderError::DuplicateWhereClause(
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
    pub fn where_clauses<I>(mut self, where_clauses: I) -> Result<Self, InternalTraitBuilderError>
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

impl Attributed for InternalTraitBuilder {
    type Attribute = InternalTraitAttribute;
}

impl IsCompleteBuilder for InternalTraitBuilder {
    fn is_complete(&self) -> bool {
        self.name.is_some() && self.publicness.is_some() && self.documentation.is_some()
    }
}

impl Builder for InternalTraitBuilder {
    type Error = BuilderError<InternalTraitAttribute>;
    type Object = InternalTrait;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(InternalTrait {
            name: self.name.ok_or(BuilderError::IncompleteBuild(InternalTraitAttribute::Name))?,
            publicness: self
                .publicness
                .ok_or(BuilderError::IncompleteBuild(InternalTraitAttribute::Publicness))?,
            methods: self.methods,
            documentation: self
                .documentation
                .ok_or(BuilderError::IncompleteBuild(InternalTraitAttribute::Documentation))?,
            where_clauses: self.where_clauses,
            generics: self.generics,
            super_traits: self.super_traits,
        })
    }
}
