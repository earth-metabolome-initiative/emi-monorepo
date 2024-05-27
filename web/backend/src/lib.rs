pub mod api;
pub mod models;
pub mod nested_models;
pub mod new_variants;
pub mod schema;
pub mod sql_function_bindings;
pub mod table_enumeration;
pub mod transactions;
pub mod update_variants;
pub mod views;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DieselConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
