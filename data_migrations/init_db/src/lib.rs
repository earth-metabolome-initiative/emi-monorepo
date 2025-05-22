#![doc = include_str!("../README.md")]

mod consistency_constraints;
mod csv_migrations;
mod errors;
mod init;
mod migrations;

pub use init::init_database;
