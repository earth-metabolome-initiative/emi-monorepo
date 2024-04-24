use web_common::api::ApiError;
use web_common::database::inserts::NewSample;

pub fn handle_new_sample(new_sample: NewSample) -> Result<(), ApiError> {
    Err(ApiError::BadRequest(vec!["Not implemented yet!".to_string()]))
}