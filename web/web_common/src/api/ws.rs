pub mod messages;
pub mod operations;
pub mod outcomes;
pub mod operation_errors;

use crate::combine_path;

pub const ENDPOINT: &str = "/ws";
pub const FULL_ENDPOINT: &str = combine_path!(super::FULL_ENDPOINT, ENDPOINT);
