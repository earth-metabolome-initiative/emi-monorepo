//! Implementation of the traits defined in the `sql_relations` crate
//! for objects related to the `sqlparser` crate.

use sql_traits::structs::ParserDB;

use crate::traits::InheritableDatabaseLike;

impl InheritableDatabaseLike for ParserDB {
    fn most_concrete_table_column_name(&self) -> &str {
        "most_concrete_table"
    }
}
