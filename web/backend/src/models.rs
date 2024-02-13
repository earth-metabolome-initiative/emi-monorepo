pub mod schema;

mod documents;
mod samples;
mod spectra;
mod spectra_collection;
mod task_type;
mod tasks;
mod taxa;
mod users;

// models.rs
use diesel::{r2d2::ConnectionManager, PgConnection};

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
