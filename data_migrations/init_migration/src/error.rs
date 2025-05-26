//! Submodule providing the errors enumeration.

use core_structures::tables::insertables::{
    InsertableAliquotingStepModelAttributes, InsertableBrandAttributes,
    InsertableCommercialProductAttributes, InsertableDocumentAttributes,
    InsertableLoginProviderAttributes, InsertableProcedureModelAttributes,
    InsertableProcedureModelContainerCategoryAttributes,
    InsertableProcedureModelNameplateCategoryAttributes,
    InsertableProcedureModelToolCategoryAttributes, InsertableReagentAttributes,
    InsertableSamplingStepModelAttributes, InsertableStepModelAttributes,
    InsertableStepModelContainerCategoryAttributes, InsertableStepModelNameplateCategoryAttributes,
    InsertableStepModelToolCategoryAttributes, InsertableStepModelTrackableCategoryAttributes,
    InsertableTrackableCategoryAttributes, InsertableUserAttributes,
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
    /// Failed to insert a new trackable category
    TrackableCategory(InsertError<InsertableTrackableCategoryAttributes>),
    /// Failed to insert a new reagent
    Reagent(InsertError<InsertableReagentAttributes>),
    /// Failed to insert a new step model
    StepModel(InsertError<InsertableStepModelAttributes>),
    /// Failed to insert a new aliquoting step model
    AliquotingStepModel(InsertError<InsertableAliquotingStepModelAttributes>),
    /// Failed to insert a new sampling step model
    SamplingStepModel(InsertError<InsertableSamplingStepModelAttributes>),
    /// Failed to insert a new document
    Document(InsertError<InsertableDocumentAttributes>),
    /// Failed to insert a new step model container category
    StepModelContainerCategory(InsertError<InsertableStepModelContainerCategoryAttributes>),
    /// Failed to insert a new step model nameplate category
    StepModelNameplateCategory(InsertError<InsertableStepModelNameplateCategoryAttributes>),
    /// Failed to insert a new step model reagent
    StepModelTrackableCategory(InsertError<InsertableStepModelTrackableCategoryAttributes>),
    /// Failed to insert a new step model tool category
    StepModelToolCategory(InsertError<InsertableStepModelToolCategoryAttributes>),
    /// Failed to insert a new procedure model container category
    ProcedureModelContainerCategory(
        InsertError<InsertableProcedureModelContainerCategoryAttributes>,
    ),
    /// Failed to insert a new procedure model nameplate category
    ProcedureModelNameplateCategory(
        InsertError<InsertableProcedureModelNameplateCategoryAttributes>,
    ),
    /// Failed to insert a new procedure model tool category
    ProcedureModelToolCategory(InsertError<InsertableProcedureModelToolCategoryAttributes>),
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

impl From<InsertError<InsertableStepModelAttributes>> for Error {
    fn from(value: InsertError<InsertableStepModelAttributes>) -> Self {
        Error::StepModel(value)
    }
}

impl From<InsertError<InsertableAliquotingStepModelAttributes>> for Error {
    fn from(value: InsertError<InsertableAliquotingStepModelAttributes>) -> Self {
        Error::AliquotingStepModel(value)
    }
}

impl From<InsertError<InsertableDocumentAttributes>> for Error {
    fn from(value: InsertError<InsertableDocumentAttributes>) -> Self {
        Error::Document(value)
    }
}

impl From<InsertError<InsertableSamplingStepModelAttributes>> for Error {
    fn from(value: InsertError<InsertableSamplingStepModelAttributes>) -> Self {
        Error::SamplingStepModel(value)
    }
}

impl From<InsertError<InsertableStepModelContainerCategoryAttributes>> for Error {
    fn from(value: InsertError<InsertableStepModelContainerCategoryAttributes>) -> Self {
        Error::StepModelContainerCategory(value)
    }
}

impl From<InsertError<InsertableStepModelTrackableCategoryAttributes>> for Error {
    fn from(value: InsertError<InsertableStepModelTrackableCategoryAttributes>) -> Self {
        Error::StepModelTrackableCategory(value)
    }
}

impl From<InsertError<InsertableProcedureModelContainerCategoryAttributes>> for Error {
    fn from(value: InsertError<InsertableProcedureModelContainerCategoryAttributes>) -> Self {
        Error::ProcedureModelContainerCategory(value)
    }
}

impl From<InsertError<InsertableProcedureModelNameplateCategoryAttributes>> for Error {
    fn from(value: InsertError<InsertableProcedureModelNameplateCategoryAttributes>) -> Self {
        Error::ProcedureModelNameplateCategory(value)
    }
}

impl From<InsertError<InsertableStepModelNameplateCategoryAttributes>> for Error {
    fn from(value: InsertError<InsertableStepModelNameplateCategoryAttributes>) -> Self {
        Error::StepModelNameplateCategory(value)
    }
}

impl From<InsertError<InsertableTrackableCategoryAttributes>> for Error {
    fn from(value: InsertError<InsertableTrackableCategoryAttributes>) -> Self {
        Error::TrackableCategory(value)
    }
}

impl From<InsertError<InsertableStepModelToolCategoryAttributes>> for Error {
    fn from(value: InsertError<InsertableStepModelToolCategoryAttributes>) -> Self {
        Error::StepModelToolCategory(value)
    }
}

impl From<InsertError<InsertableProcedureModelToolCategoryAttributes>> for Error {
    fn from(value: InsertError<InsertableProcedureModelToolCategoryAttributes>) -> Self {
        Error::ProcedureModelToolCategory(value)
    }
}
