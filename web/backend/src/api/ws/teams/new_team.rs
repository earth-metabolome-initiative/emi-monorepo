use web_common::api::ApiError;
use web_common::database::inserts::NewTeam;

pub fn handle_new_team(new_team: NewTeam) -> Result<(), ApiError> {
    Err(ApiError::BadRequest(vec!["Not implemented yet!".to_string()]))
}
