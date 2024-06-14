pub mod diesel_types;
pub(crate) use diesel_types::*;
pub mod schema;
pub use schema::*;
pub mod sql_function_bindings;
pub use sql_function_bindings::*;
pub mod sql_operator_bindings;
pub use sql_operator_bindings::*;
pub mod sql_type_bindings;

use diesel::r2d2::{self, ConnectionManager, Pool};
pub type DBPool = Pool<ConnectionManager<diesel::PgConnection>>;
pub type DieselConn = r2d2::PooledConnection<ConnectionManager<diesel::PgConnection>>;

pub mod update_variants;
pub(crate) use update_variants::*;
pub mod flat_variants;
pub use flat_variants::*;
pub mod nested_variants;
pub use nested_variants::*;
pub mod new_variants;
pub(crate) use new_variants::*;
pub mod table_enumeration;
pub use table_enumeration::*;
