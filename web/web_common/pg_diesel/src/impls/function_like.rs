//! Submodule implementing the
//! [`FunctionLike`](sql_traits::prelude::FunctionLike) trait for the
//! [`PgProc`](crate::models::PgProc) struct.

use sql_traits::traits::{FunctionLike, Metadata};

use crate::{database::PgProcMetadata, models::PgProc, PgDatabase};

impl Metadata for PgProc {
    type Meta = PgProcMetadata;
}

impl FunctionLike for PgProc {
    type Database = PgDatabase;

    fn name(&self) -> &str {
        &self.proname
    }

    fn argument_type_names(&self, database: &Self::Database) -> Vec<String> {
        database
            .function_metadata(self)
            .argument_types()
            .iter()
            .map(|pg_type| pg_type.typname.clone())
            .collect()
    }

    fn return_type_name(&self, database: &Self::Database) -> Option<String> {
        database
            .function_metadata(self)
            .return_type()
            .map(|pg_type| pg_type.typname.clone())
    }
}
