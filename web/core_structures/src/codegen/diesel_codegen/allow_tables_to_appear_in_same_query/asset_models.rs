use crate::codegen::diesel_codegen::tables::{asset_models::asset_models, users::users};
diesel::allow_tables_to_appear_in_same_query!(asset_models, users);
