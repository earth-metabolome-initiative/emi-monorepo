use web_common::api::ApiError;
use web_common::database::inserts::NewProject;

pub fn handle_new_project(new_project: NewProject) -> Result<(), ApiError> {
    Err(ApiError::internal_server_error())
}
