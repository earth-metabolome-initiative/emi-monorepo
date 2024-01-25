pub mod schema;

mod documents;
mod samples;
mod users;
mod spectra_collection;
mod spectra;
mod task_type;
mod tasks;
mod taxons;

// models.rs
use diesel::{r2d2::ConnectionManager, PgConnection};

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;