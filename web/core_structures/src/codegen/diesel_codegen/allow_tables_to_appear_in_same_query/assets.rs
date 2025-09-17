use crate::codegen::diesel_codegen::tables::{asset_models::asset_models, assets::assets};
diesel::allow_tables_to_appear_in_same_query!(assets, asset_models);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(assets, users);
