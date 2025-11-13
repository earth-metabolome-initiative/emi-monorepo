//! Implementation of [`FunctionLike`] for [`PgProc`].
//!
//! This module implements the
//! [`FunctionLike`](sql_traits::prelude::FunctionLike) trait
//! for the [`PgProc`](crate::models::PgProc) model from `pg_catalog.pg_proc`,
//! enabling generic introspection of PostgreSQL functions and procedures.
//!
//! The implementation provides access to:
//! - Function name
//! - Argument type names (resolved from `pg_type`)
//! - Return type name (resolved from `pg_type`)
//!
//! Type information is loaded from [`PgProcMetadata`] which resolves OIDs to
//! type names.

use sql_traits::traits::{FunctionLike, Metadata};

use crate::{PgDatabase, database::PgProcMetadata, models::PgProc};

impl Metadata for PgProc {
    type Meta = PgProcMetadata;
}

impl FunctionLike for PgProc {
    type DB = PgDatabase;

    fn name(&self) -> &str {
        &self.proname
    }

    fn argument_type_names<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db str> {
        database
            .function_metadata(self)
            .expect("Function must exist in database")
            .argument_types()
            .iter()
            .map(|pg_type| pg_type.typname.as_str())
    }

    fn return_type_name<'db>(&'db self, database: &'db Self::DB) -> Option<&'db str> {
        database
            .function_metadata(self)
            .expect("Function must exist in database")
            .return_type()
            .map(|pg_type| pg_type.typname.as_str())
    }
}
