#![doc = include_str!("../README.md")]

pub mod attributes;
pub mod crud;
pub mod database;
pub mod filtrable;
pub mod insert_operation;
pub mod operation;
pub mod operation_error;
pub mod outcome;
pub mod session;
pub mod session_operation;

/// Main prelude for the library.
pub mod prelude {
    pub use common_traits::prelude::*;

    #[cfg(feature = "postgres")]
    pub use crate::database::UncheckedInsertableVariant;
    pub use crate::{
        attributes::*,
        database::{
            Ancestor, AncestorExists, BoundedRead, BoundedReadDispatch, Deletable, Descendant,
            ExtensionTable, ForeignKeys, HasForeignKeys, Insertable, InsertableVariant,
            ReadDispatch, Row, Rows, SetPrimaryKey, StaticTabular, TableName, Tabular, UpsertVec,
            Upsertable,
        },
        filtrable::*,
        operation::Operation,
        operation_error::OperationError,
        outcome::Outcome,
        session::Session,
        session_operation::SessionOperation,
    };
}
