//! Submodule defining the errors which might occur when using the
//! `GuidedProcedure`.

use std::fmt::Display;

use core_structures::{ProcedureTemplate, tables::most_concrete_variants::ProcedureInsertErrorDAG};

#[derive(Debug)]
/// Enum representing the possible errors which might occur when using the
/// `GuidedProcedure`.
pub enum InternalGuidedProcedureError<'graph> {
    /// When the provided designated successor was not found in the viable
    /// successors.
    DesignatedSuccessorNotFound {
        designated_successor: &'graph ProcedureTemplate,
        viable_successors: Vec<&'graph ProcedureTemplate>,
    },
    /// When no designated successor was provided but multiple viable successors
    /// were found.
    UnclearSuccessor { viable_successors: Vec<&'graph ProcedureTemplate> },
    /// Unexpected builder type encountered.
    UnexpectedBuilder {
        expected: &'static str,
        found: &'static str,
        template: &'graph ProcedureTemplate,
    },
    /// No more builders are available.
    NoMoreBuilders,
    /// A builder was not yet processed.
    UnprocessedBuilder { found: &'static str, template: &'graph ProcedureTemplate },
    /// An insertion failed because a unique constraint was violated.
    Insert(ProcedureInsertErrorDAG),
    /// An error occurred while interacting with the database.
    Diesel(diesel::result::Error),
}

#[derive(Debug)]
/// Enum representing the possible errors which might occur when using the
/// `GuidedProcedure`.
pub enum GuidedProcedureError {
    /// No more builders are available.
    NoMoreBuilders,
    /// An insertion failed because a unique constraint was violated.
    Insert(ProcedureInsertErrorDAG),
    /// An error occurred while interacting with the database.
    Diesel(diesel::result::Error),
}

impl From<diesel::result::Error> for InternalGuidedProcedureError<'_> {
    fn from(e: diesel::result::Error) -> Self {
        InternalGuidedProcedureError::Diesel(e)
    }
}

impl From<diesel::result::Error> for GuidedProcedureError {
    fn from(e: diesel::result::Error) -> Self {
        GuidedProcedureError::Diesel(e)
    }
}

impl From<ProcedureInsertErrorDAG> for InternalGuidedProcedureError<'_> {
    fn from(e: ProcedureInsertErrorDAG) -> Self {
        InternalGuidedProcedureError::Insert(e)
    }
}

impl From<ProcedureInsertErrorDAG> for GuidedProcedureError {
    fn from(e: ProcedureInsertErrorDAG) -> Self {
        GuidedProcedureError::Insert(e)
    }
}

impl Display for InternalGuidedProcedureError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DesignatedSuccessorNotFound { designated_successor, viable_successors } => {
                write!(
                    f,
                    "The designated successor '{}' was not found in the viable successors: [{}]",
                    designated_successor.name,
                    viable_successors
                        .iter()
                        .map(|pt| pt.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Self::UnclearSuccessor { viable_successors } => {
                write!(
                    f,
                    "Multiple viable successors were found: [{}]. Please specify a designated successor.",
                    viable_successors
                        .iter()
                        .map(|pt| pt.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Self::UnexpectedBuilder { expected, found, template } => {
                write!(
                    f,
                    "Expected builder of type `{expected}`, but a builder of type `{found}` is required to build the procedure template \"{}\" from table \"{}\".",
                    template.name, template.most_concrete_table
                )
            }
            Self::UnprocessedBuilder { found, template } => {
                write!(
                    f,
                    "A builder of type `{found}` was not yet processed for the procedure template \"{}\" from table \"{}\". You most likely need to add another `.and_then(|builder, conn| {{...}})` to your guided procedure.",
                    template.name, template.most_concrete_table
                )
            }
            Self::NoMoreBuilders => {
                write!(f, "No more builders are available.")
            }
            Self::Insert(e) => write!(f, "An insertion error occurred: {e}"),
            Self::Diesel(e) => write!(f, "A database error occurred: {e}"),
        }
    }
}

impl Display for GuidedProcedureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoMoreBuilders => {
                write!(f, "No more builders are available.")
            }
            Self::Insert(e) => write!(f, "An insertion error occurred: {e}"),
            Self::Diesel(e) => write!(f, "A database error occurred: {e}"),
        }
    }
}

impl std::error::Error for InternalGuidedProcedureError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Diesel(e) => Some(e),
            Self::Insert(e) => Some(e),
            _ => None,
        }
    }
}

impl std::error::Error for GuidedProcedureError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Diesel(e) => Some(e),
            Self::Insert(e) => Some(e),
            Self::NoMoreBuilders => None,
        }
    }
}
