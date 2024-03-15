pub mod messages;
use crate::combine_path;

pub const ENDPOINT: &str = "/ws";
pub const FULL_ENDPOINT: &str = combine_path!(super::FULL_ENDPOINT, ENDPOINT);
