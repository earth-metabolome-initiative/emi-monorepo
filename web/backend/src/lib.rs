pub mod api;
pub mod model_implementations;
pub mod models;
pub mod nested_models;
pub mod schema;
pub mod transactions;
pub mod views;
pub mod table_enumeration;
pub mod new_variants;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DieselConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
