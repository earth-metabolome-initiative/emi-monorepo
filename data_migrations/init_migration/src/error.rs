//! Submodule providing the errors enumeration.

use core_structures::tables::insertables::{
    InsertableBrandAttributes, InsertableCommercialProductAttributes, InsertableDocumentAttributes,
    InsertableLoginProviderAttributes, InsertableParentProcedureModelAttributes,
    InsertableProcedureModelAttributes, InsertableReagentAttributes, InsertableUserAttributes,
};
use web_common_traits::database::InsertError;

#[derive(Debug)]
#[allow(dead_code)]
/// Error enumeration for the `init_migration` module.
pub enum Error {
    /// Failed to establish database connection
    ConnectionFailed(diesel::ConnectionError),
    /// Failed to execute a query
    QueryFailed(diesel::result::Error),
    /// Failed to insert a new login provider
    LoginProvider(InsertError<InsertableLoginProviderAttributes>),
    /// Failed to insert a new procedure model
    ProcedureModel(InsertError<InsertableProcedureModelAttributes>),
    /// Failed to insert a new commercial product
    CommercialProduct(InsertError<InsertableCommercialProductAttributes>),
    /// Failed to insert a new brand
    Brand(InsertError<InsertableBrandAttributes>),
    /// Failed to insert a  new user
    User(InsertError<InsertableUserAttributes>),
    /// Failed to insert a new reagent
    Reagent(InsertError<InsertableReagentAttributes>),
    /// Failed to insert a new document
    Document(InsertError<InsertableDocumentAttributes>),
    /// Failed to insert a parent-child relationship
    ParentProcedureModel(InsertError<InsertableParentProcedureModelAttributes>),
}

impl From<diesel::ConnectionError> for Error {
    fn from(value: diesel::ConnectionError) -> Self {
        Error::ConnectionFailed(value)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        Error::QueryFailed(value)
    }
}

impl From<InsertError<InsertableLoginProviderAttributes>> for Error {
    fn from(value: InsertError<InsertableLoginProviderAttributes>) -> Self {
        Error::LoginProvider(value)
    }
}

impl From<InsertError<InsertableProcedureModelAttributes>> for Error {
    fn from(value: InsertError<InsertableProcedureModelAttributes>) -> Self {
        Error::ProcedureModel(value)
    }
}

impl From<InsertError<InsertableUserAttributes>> for Error {
    fn from(value: InsertError<InsertableUserAttributes>) -> Self {
        Error::User(value)
    }
}

impl From<InsertError<InsertableCommercialProductAttributes>> for Error {
    fn from(value: InsertError<InsertableCommercialProductAttributes>) -> Self {
        Error::CommercialProduct(value)
    }
}

impl From<InsertError<InsertableBrandAttributes>> for Error {
    fn from(value: InsertError<InsertableBrandAttributes>) -> Self {
        Error::Brand(value)
    }
}

impl From<InsertError<InsertableReagentAttributes>> for Error {
    fn from(value: InsertError<InsertableReagentAttributes>) -> Self {
        Error::Reagent(value)
    }
}

impl From<InsertError<InsertableDocumentAttributes>> for Error {
    fn from(value: InsertError<InsertableDocumentAttributes>) -> Self {
        Error::Document(value)
    }
}

impl From<InsertError<InsertableParentProcedureModelAttributes>> for Error {
    fn from(value: InsertError<InsertableParentProcedureModelAttributes>) -> Self {
        Error::ParentProcedureModel(value)
    }
}
