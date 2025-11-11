//! Submodule providing a variant of the `InternalToken` builder which
//! helps to implement completely an `InternalTrait` in meta-programming.

use std::{collections::HashSet, fmt::Display};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};
use quote::quote;

use crate::structs::{
    DataVariantRef, InternalToken, Method, WhereClause,
    external_trait::TraitVariantRef,
    internal_token::{
        InternalTokenBuilder,
        builder::{InternalTokenAttribute, InternalTokenBuilderError},
    },
};

/// Struct to help implement a trait into an `InternalToken`.
pub struct TraitImpl<'trt> {
    /// The underlying internal token builder.
    builder: InternalTokenBuilder,
    /// The trait to implement.
    trait_ref: &'trt TraitVariantRef,
    /// The methods defined by the user, to be added to the token stream.
    methods: Vec<Method>,
    /// The type for which the trait is being implemented.
    data: Option<&'trt DataVariantRef>,
    /// Where clauses for the implementation.
    where_clauses: Vec<WhereClause>,
}

impl<'trt> TraitImpl<'trt> {
    /// Creates a new `TraitImpl` instance.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait to implement.
    pub fn new(trait_ref: &'trt TraitVariantRef) -> Self {
        Self {
            builder: InternalToken::new(),
            trait_ref,
            methods: Vec::new(),
            data: None,
            where_clauses: Vec::new(),
        }
    }

    /// Sets the type for which the trait is being implemented.
    ///
    /// # Arguments
    /// * `data` - The type for which the trait is being implemented.
    pub fn for_type(mut self, data: &'trt DataVariantRef) -> Self {
        self.data = Some(data);
        self
    }

    /// Adds a method to the trait implementation.
    ///
    /// # Arguments
    ///
    /// * `method` - The method to add.
    ///
    /// # Errors
    ///
    /// * If the provided method is already defined.
    /// * If the provided method does not belong to the trait being implemented.
    /// * If the provided method is missing the body.
    /// * If the provided method has a documentation field.
    /// * If the provided method has a visibility other than private.
    /// * If the provided method is incompatible with the curresponding method
    ///   in the trait.
    pub fn method(mut self, method: Method) -> Result<Self, TraitImplError> {
        if self.methods.iter().any(|m| m.name() == method.name()) {
            return Err(TraitImplError::MethodAlreadyDefined(method.signature()));
        }
        if !self.trait_ref.defines_method(&method) {
            return Err(TraitImplError::MethodSignatureMismatch(method.signature()));
        }
        if !method.has_body() {
            return Err(TraitImplError::MethodWithoutBody(method.signature()));
        }

        self.methods.push(method);
        Ok(self)
    }

    /// Adds several methods to the trait implementation.
    ///
    /// # Arguments
    ///
    /// * `methods` - The methods to add.
    pub fn methods<I>(mut self, methods: I) -> Result<Self, TraitImplError>
    where
        I: IntoIterator<Item = Method>,
    {
        for method in methods {
            self = self.method(method)?;
        }

        Ok(self)
    }

    /// Adds a where clause to the trait implementation.
    ///
    /// # Arguments
    ///
    /// * `where_clause` - The where clause to add.
    ///
    /// # Errors
    ///
    /// * If the provided where clause is already defined.
    pub fn where_clause(mut self, where_clause: WhereClause) -> Result<Self, TraitImplError> {
        if self.where_clauses.iter().any(|wc| wc == &where_clause) {
            return Err(TraitImplError::DuplicateWhereClause(where_clause.to_string()));
        }
        self.where_clauses.push(where_clause);
        Ok(self)
    }
}

#[derive(Debug)]
/// Errors that can occur when building a `TraitImpl`.
pub enum TraitImplError {
    /// Some error occurred in the internal token builder.
    Builder(InternalTokenBuilderError),
    /// A required method for the trait implementation is missing.
    RequiredMethodMissing(String),
    /// A method was provided without a body.
    MethodWithoutBody(String),
    /// A method was provided with incompatible signature.
    MethodSignatureMismatch(String),
    /// A method was provided multiple times.
    MethodAlreadyDefined(String),
    /// The data type for which the trait is being implemented is missing.
    MissingDataType,
    /// A where clause was duplicated.
    DuplicateWhereClause(String),
}

impl Display for TraitImplError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TraitImplError::Builder(err) => write!(f, "TraitImpl builder error: {}", err),
            TraitImplError::RequiredMethodMissing(method_name) => {
                write!(f, "Required method '{}' is missing for trait implementation", method_name)
            }
            TraitImplError::MethodWithoutBody(method_name) => {
                write!(f, "Method '{}' was provided without a body", method_name)
            }
            TraitImplError::MethodSignatureMismatch(method_name) => {
                write!(f, "Method '{}' has a signature incompatible with the trait", method_name)
            }
            TraitImplError::MethodAlreadyDefined(method_name) => {
                write!(
                    f,
                    "Method '{}' was already defined in the trait implementation",
                    method_name
                )
            }
            TraitImplError::MissingDataType => {
                write!(f, "The data type for which the trait is being implemented is missing")
            }
            TraitImplError::DuplicateWhereClause(where_clause) => {
                write!(
                    f,
                    "The where clause '{}' was duplicated in the trait implementation",
                    where_clause
                )
            }
        }
    }
}

impl std::error::Error for TraitImplError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TraitImplError::Builder(err) => Some(err),
            _ => None,
        }
    }
}

impl From<InternalTokenBuilderError> for TraitImplError {
    fn from(err: InternalTokenBuilderError) -> Self {
        TraitImplError::Builder(err)
    }
}

impl From<BuilderError<InternalTokenAttribute>> for TraitImplError {
    fn from(err: BuilderError<InternalTokenAttribute>) -> Self {
        let err: InternalTokenBuilderError = err.into();
        err.into()
    }
}

impl<'trt> Attributed for TraitImpl<'trt> {
    type Attribute = InternalTokenAttribute;
}

impl<'trt> IsCompleteBuilder for TraitImpl<'trt> {
    fn is_complete(&self) -> bool {
        self.builder.is_complete()
    }
}

impl<'trt> TryFrom<TraitImpl<'trt>> for InternalToken {
    type Error = TraitImplError;

    fn try_from(value: TraitImpl<'trt>) -> Result<Self, Self::Error> {
        // Ensure all required methods are provided.
        for method in value.trait_ref.methods() {
            // Method with default implementation can be skipped.
            if method.has_body() {
                continue;
            }
            // Check if the method is provided.
            if !value.methods.iter().any(|m| m.name() == method.name()) {
                return Err(TraitImplError::RequiredMethodMissing(method.signature()));
            }
        }

        let mut unique_types: HashSet<DataVariantRef> = HashSet::new();
        let methods = value.methods;
        for provided_method in methods.iter() {
            for arg in provided_method.arguments() {
                unique_types.insert(arg.arg_type().clone());
            }
            if let Some(ret_type) = provided_method.return_type() {
                unique_types.insert(ret_type.clone());
            }
        }

        let trait_ref = &value.trait_ref;
        let data = value.data.ok_or(TraitImplError::MissingDataType)?;
        let formatted_where_clauses = if value.where_clauses.is_empty() {
            None
        } else {
            let clauses = &value.where_clauses;
            Some(quote! { where #(#clauses),* })
        };

        let data_with_generics = data.format_with_generics();
        let generics_without_defaults = data.generics_without_defaults();
        let formatted_generics_without_defaults = if generics_without_defaults.is_empty() {
            quote! {}
        } else {
            quote! { <#(#generics_without_defaults),*> }
        };

        Ok(value
            .builder
            .private()
            .employed_trait(value.trait_ref.clone())
            .data(data.clone())
            .datas(unique_types)
            .stream(quote! {
                impl #formatted_generics_without_defaults #trait_ref for #data_with_generics #formatted_where_clauses {
                    #(#methods)*
                }
            })
            .inherits(methods.iter().flat_map(|method| method.where_clauses().flat_map(|clause| vec![clause.left().clone(), clause.right().clone()])))
            .inherits(methods.iter().filter_map(|method| method.body().cloned()))
            .build()?)
    }
}
