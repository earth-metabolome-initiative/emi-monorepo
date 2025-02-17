pub mod messages;
pub mod operation_errors;
pub mod operations;
pub mod outcomes;

use crate::combine_path;

pub const ENDPOINT: &str = "/ws";
pub const FULL_ENDPOINT: &str = combine_path!(super::FULL_ENDPOINT, ENDPOINT);
