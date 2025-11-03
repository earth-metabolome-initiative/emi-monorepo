//! Submodule defining a builder for the `InternalTrait` struct.

use std::{error::Error, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};
use quote::ToTokens;

use crate::structs::{
    Documentation, InternalToken, InternalTrait, Publicness, WhereClause, internal_struct::Method,
};

#[derive(Default)]
/// Builder for the `InternalTrait` struct.
pub struct InternalTraitBuilder<'data> {
    /// Name of the trait.
    name: Option<String>,
    /// Publicness of the trait.
    publicness: Option<Publicness>,
    /// Internal token streams defined within the trait.
    methods: Vec<Method<'data>>,
    /// Trait documentation.
    documentation: Option<Documentation<'data>>,
    /// Where statements for the trait.
    where_statements: Vec<WhereClause<'data>>,
    /// Generics for the trait.
    generics: Vec<syn::Ident>,
    /// Super traits for the trait.
    super_traits: Vec<InternalToken<'data>>,
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    /// Duplicate super trait found.
    DuplicateSuperTrait(String),
}

impl Display for InternalTraitBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalTraitBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            InternalTraitBuilderError::InvalidName => write!(f, "Invalid trait name"),
            InternalTraitBuilderError::DuplicateMethodName(name) => {
                write!(f, "Duplicate method name found in trait: {}", name)
            }
            InternalTraitBuilderError::DuplicateWhereClause(clause) => {
                write!(f, "Duplicate where clause found in trait: {}", clause)
            }
            InternalTraitBuilderError::DuplicateGeneric(generic) => {
                write!(f, "Duplicate generic found in trait: {}", generic)
            }
            InternalTraitBuilderError::DuplicateSuperTrait(trait_name) => {
                write!(f, "Duplicate super trait found in trait: {}", trait_name)
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

impl<'data> InternalTraitBuilder<'data> {
    /// Sets the name of the trait.
    ///
    /// # Arguments
    /// * `name` - The name of the trait.
    pub fn name<S: ToString>(mut self, name: S) -> Result<Self, InternalTraitBuilderError> {
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
    pub fn documentation(mut self, documentation: Documentation<'data>) -> Self {
        self.documentation = Some(documentation);
        self
    }

    /// Adds a method to the trait.
    ///
    /// # Arguments
    /// * `method` - The method to add.
    pub fn method(mut self, method: Method<'data>) -> Result<Self, InternalTraitBuilderError> {
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
        I: IntoIterator<Item = Method<'data>>,
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
    pub fn generic(mut self, generic: syn::Ident) -> Result<Self, InternalTraitBuilderError> {
        if self.generics.contains(&generic) {
            return Err(InternalTraitBuilderError::DuplicateGeneric(generic.to_string()));
        }
        self.generics.push(generic);
        Ok(self)
    }

    /// Adds several generics to the trait.
    ///
    /// # Arguments
    /// * `generics` - The generics to add.
    pub fn generics<I>(mut self, generics: I) -> Result<Self, InternalTraitBuilderError>
    where
        I: IntoIterator<Item = syn::Ident>,
    {
        for generic in generics {
            self = self.generic(generic)?;
        }
        Ok(self)
    }

    /// Adds a where clause to the trait.
    ///
    /// # Arguments
    /// * `where_clause` - The where clause to add.
    pub fn where_clause(
        mut self,
        where_clause: WhereClause<'data>,
    ) -> Result<Self, InternalTraitBuilderError> {
        if self.where_statements.contains(&where_clause) {
            return Err(InternalTraitBuilderError::DuplicateWhereClause(
                where_clause.to_token_stream().to_string(),
            ));
        }
        self.where_statements.push(where_clause);
        Ok(self)
    }

    /// Adds several where clauses to the trait.
    ///
    /// # Arguments
    /// * `where_clauses` - The where clauses to add.
    pub fn where_clauses<I>(mut self, where_clauses: I) -> Result<Self, InternalTraitBuilderError>
    where
        I: IntoIterator<Item = WhereClause<'data>>,
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
    pub fn super_trait(
        mut self,
        super_trait: InternalToken<'data>,
    ) -> Result<Self, InternalTraitBuilderError> {
        if self.super_traits.contains(&super_trait) {
            return Err(InternalTraitBuilderError::DuplicateSuperTrait(
                super_trait.to_token_stream().to_string(),
            ));
        }
        self.super_traits.push(super_trait);
        Ok(self)
    }

    /// Adds several super traits to the trait.
    ///
    /// # Arguments
    /// * `super_traits` - The super traits to add.
    pub fn super_traits<I>(mut self, super_traits: I) -> Result<Self, InternalTraitBuilderError>
    where
        I: IntoIterator<Item = InternalToken<'data>>,
    {
        for super_trait in super_traits {
            self = self.super_trait(super_trait)?;
        }
        Ok(self)
    }

    /// Adds the `Sized` super-trait to the trait.
    pub fn sized(self) -> Result<Self, InternalTraitBuilderError> {
        self.super_trait(
            InternalToken::new()
                .private()
                .stream(quote::quote! { Sized })
                .build()
                .expect("Failed to build InternalToken for Sized super-trait"),
        )
    }
}

impl Attributed for InternalTraitBuilder<'_> {
    type Attribute = InternalTraitAttribute;
}

impl IsCompleteBuilder for InternalTraitBuilder<'_> {
    fn is_complete(&self) -> bool {
        self.name.is_some() && self.publicness.is_some() && self.documentation.is_some()
    }
}

impl<'data> Builder for InternalTraitBuilder<'data> {
    type Error = BuilderError<InternalTraitAttribute>;
    type Object = InternalTrait<'data>;

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
            where_statements: self.where_statements,
            generics: self.generics,
            super_traits: self.super_traits,
        })
    }
}
