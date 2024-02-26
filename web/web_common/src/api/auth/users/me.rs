use crate::combine_path;

pub const ENDPOINT: &str = "/me";
pub const FULL_ENDPOINT: &str = combine_path!(super::FULL_ENDPOINT, ENDPOINT);
