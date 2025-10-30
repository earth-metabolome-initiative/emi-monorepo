//! Submodule defining a builder for the `Method` struct.

use std::{error::Error, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};
use syn::Ident;

use crate::structs::{
    Argument, Documentation, InternalToken, Method, Publicness, WhereClause,
    internal_data::DataVariantRef,
};

#[derive(Default)]
/// Builder for the `Method` struct.
pub struct MethodBuilder<'data> {
    /// Arguments of the method.
    arguments: Vec<Argument<'data>>,
    /// Name of the method.
    name: Option<String>,
    /// Publicness of the method.
    publicness: Option<Publicness>,
    /// The body of the method.
    body: Option<InternalToken<'data>>,
    /// Whether the method is asynchronous.
    async_method: bool,
    /// The return type of the method.
    return_type: Option<DataVariantRef<'data>>,
    /// Documentation of the method.
    documentation: Option<Documentation<'data>>,
    /// Generics of the method.
    generics: Vec<Ident>,
    /// Where clauses of the method.
    where_clauses: Vec<WhereClause<'data>>,
    /// Error documentation of the method.
    error_documentation: Option<Documentation<'data>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of the attributes of the `Method` struct.
pub enum MethodAttribute {
    /// Arguments of the method.
    Arguments,
    /// Name of the method.
    Name,
    /// Publicness of the method.
    Publicness,
    /// The body of the method.
    Body,
    /// Whether the method is asynchronous.
    AsyncMethod,
    /// The return type of the method.
    ReturnType,
    /// Documentation of the method.
    Documentation,
    /// Generics of the method.
    Generics,
    /// Where clauses of the method.
    WhereClauses,
    /// Error documentation of the method.
    ErrorDocumentation,
}

impl Display for MethodAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MethodAttribute::Arguments => write!(f, "arguments"),
            MethodAttribute::Name => write!(f, "name"),
            MethodAttribute::Publicness => write!(f, "publicness"),
            MethodAttribute::Body => write!(f, "body"),
            MethodAttribute::AsyncMethod => write!(f, "async_method"),
            MethodAttribute::ReturnType => write!(f, "return_type"),
            MethodAttribute::Documentation => write!(f, "documentation"),
            MethodAttribute::Generics => write!(f, "generics"),
            MethodAttribute::WhereClauses => write!(f, "where_clauses"),
            MethodAttribute::ErrorDocumentation => write!(f, "error_documentation"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration of errors that can occur during the building of a
/// `Method`.
pub enum MethodBuilderError {
    /// An error occurred during the building process.
    Builder(BuilderError<MethodAttribute>),
    /// The name of the method is invalid.
    InvalidName,
    /// An argument with the same name has already been added.
    DuplicatedArgument,
    /// A where clause with the same left-hand side has already been added.
    DuplicatedWhereClause,
    /// A generic with the same name has already been added.
    DuplicatedGeneric,
}

impl From<BuilderError<MethodAttribute>> for MethodBuilderError {
    fn from(err: BuilderError<MethodAttribute>) -> Self {
        MethodBuilderError::Builder(err)
    }
}

impl Display for MethodBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MethodBuilderError::Builder(e) => write!(f, "Builder error: {}", e),
            MethodBuilderError::InvalidName => write!(f, "Invalid method name"),
            MethodBuilderError::DuplicatedArgument => {
                write!(f, "An argument with the same name has already been added")
            }
            MethodBuilderError::DuplicatedWhereClause => {
                write!(f, "A where clause with the same left-hand side has already been added")
            }
            MethodBuilderError::DuplicatedGeneric => {
                write!(f, "A generic with the same name has already been added")
            }
        }
    }
}

impl Error for MethodBuilderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MethodBuilderError::Builder(e) => Some(e),
            _ => None,
        }
    }
}

impl<'data> MethodBuilder<'data> {
    /// Sets the name of the method.
    ///
    /// # Arguments
    /// * `name` - The name of the method.
    pub fn name<S: ToString>(mut self, name: S) -> Result<Self, MethodBuilderError> {
        let name = name.to_string();
        if name.trim().is_empty()
            || name.contains(' ')
            || !name.chars().all(|c| c.is_alphanumeric() || c == '_')
        {
            return Err(MethodBuilderError::InvalidName);
        }
        self.name = Some(name);
        Ok(self)
    }

    /// Sets the documentation of the method.
    ///
    /// # Arguments
    /// * `documentation` - The documentation of the method.
    pub fn documentation(mut self, documentation: Documentation<'data>) -> Self {
        self.documentation = Some(documentation);
        self
    }

    /// Sets the publicness of the method.
    ///
    /// # Arguments
    /// * `publicness` - The publicness of the method.
    pub fn publicness(mut self, publicness: Publicness) -> Self {
        self.publicness = Some(publicness);
        self
    }

    /// Sets the method as public.
    pub fn public(mut self) -> Self {
        self.publicness = Some(Publicness::Public);
        self
    }

    /// Sets the method as private.
    pub fn private(mut self) -> Self {
        self.publicness = Some(Publicness::Private);
        self
    }

    /// Sets the body of the method.
    ///
    /// # Arguments
    /// * `body` - The body of the method.
    pub fn body<T>(mut self, body: T) -> Self
    where
        T: Into<InternalToken<'data>>,
    {
        self.body = Some(body.into());
        self
    }

    /// Sets whether the method is asynchronous.
    ///
    /// # Arguments
    /// * `async_method` - Whether the method is asynchronous.
    pub fn async_method(mut self, async_method: bool) -> Self {
        self.async_method = async_method;
        self
    }

    /// Sets the return type of the method.
    ///
    /// # Arguments
    /// * `return_type` - The return type of the method.
    pub fn return_type<T>(mut self, return_type: T) -> Self
    where
        T: Into<DataVariantRef<'data>>,
    {
        self.return_type = Some(return_type.into());
        self
    }

    /// Adds an argument to the method.
    ///
    /// # Arguments
    /// * `argument` - The argument to add.
    pub fn add_argument(mut self, argument: Argument<'data>) -> Result<Self, MethodBuilderError> {
        if self.arguments.iter().any(|a| a.name() == argument.name()) {
            return Err(MethodBuilderError::DuplicatedArgument);
        }
        self.arguments.push(argument);
        Ok(self)
    }

    /// Adds multiple arguments to the method.
    ///
    /// # Arguments
    /// * `arguments` - The arguments to add.
    pub fn add_arguments<I>(mut self, arguments: I) -> Result<Self, MethodBuilderError>
    where
        I: IntoIterator<Item = Argument<'data>>,
    {
        for argument in arguments {
            self = self.add_argument(argument)?;
        }
        Ok(self)
    }

    /// Adds a generic to the method.
    ///
    /// # Arguments
    /// * `generic` - The generic to add.
    pub fn add_generic(mut self, generic: Ident) -> Result<Self, MethodBuilderError> {
        if self.generics.iter().any(|g| g == &generic) {
            return Err(MethodBuilderError::DuplicatedGeneric);
        }
        self.generics.push(generic);
        Ok(self)
    }

    /// Adds multiple generics to the method.
    ///
    /// # Arguments
    /// * `generics` - The generics to add.
    pub fn add_generics<I>(mut self, generics: I) -> Result<Self, MethodBuilderError>
    where
        I: IntoIterator<Item = Ident>,
    {
        for generic in generics {
            self = self.add_generic(generic)?;
        }
        Ok(self)
    }

    /// Adds a where clause to the method.
    ///
    /// # Arguments
    /// * `where_clause` - The where clause to add.
    pub fn add_where_clause(
        mut self,
        where_clause: WhereClause<'data>,
    ) -> Result<Self, MethodBuilderError> {
        if self.where_clauses.iter().any(|w| w == &where_clause) {
            return Err(MethodBuilderError::DuplicatedWhereClause);
        }
        self.where_clauses.push(where_clause);
        Ok(self)
    }

    /// Adds multiple where clauses to the method.
    ///
    /// # Arguments
    /// * `where_clauses` - The where clauses to add.
    pub fn add_where_clauses<I>(mut self, where_clauses: I) -> Result<Self, MethodBuilderError>
    where
        I: IntoIterator<Item = WhereClause<'data>>,
    {
        for where_clause in where_clauses {
            self = self.add_where_clause(where_clause)?;
        }
        Ok(self)
    }

    /// Sets the error documentation of the method.
    ///
    /// # Arguments
    /// * `error_documentation` - The error documentation of the method.
    pub fn error_documentation(mut self, error_documentation: Documentation<'data>) -> Self {
        self.error_documentation = Some(error_documentation);
        self
    }
}

impl Attributed for MethodBuilder<'_> {
    type Attribute = MethodAttribute;
}

impl IsCompleteBuilder for MethodBuilder<'_> {
    fn is_complete(&self) -> bool {
        self.name.is_some() && self.publicness.is_some() && self.documentation.is_some()
    }
}

impl<'data> Builder for MethodBuilder<'data> {
    type Error = MethodBuilderError;
    type Object = Method<'data>;

    fn build(self) -> Result<Self::Object, Self::Error> {
        if self.return_type.as_ref().map_or(false, |rt| rt.is_result())
            && self.error_documentation.is_none()
        {
            return Err(BuilderError::IncompleteBuild(MethodAttribute::ErrorDocumentation).into());
        } else if self.return_type.as_ref().map_or(true, |rt| !rt.is_result())
            && self.error_documentation.is_some()
        {
            return Err(
                BuilderError::UnexpectedAttribute(MethodAttribute::ErrorDocumentation).into()
            );
        }

        Ok(Method {
            arguments: self.arguments,
            name: self.name.ok_or(BuilderError::IncompleteBuild(MethodAttribute::Name))?,
            publicness: self
                .publicness
                .ok_or(BuilderError::IncompleteBuild(MethodAttribute::Publicness))?,
            body: self.body,
            async_method: self.async_method,
            return_type: self.return_type,
            documentation: self
                .documentation
                .ok_or(BuilderError::IncompleteBuild(MethodAttribute::Documentation))?,
            generics: self.generics,
            where_clauses: self.where_clauses,
            error_documentation: self.error_documentation,
        })
    }
}
