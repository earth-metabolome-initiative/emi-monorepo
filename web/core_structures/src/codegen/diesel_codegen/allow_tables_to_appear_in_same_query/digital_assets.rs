use crate::codegen::diesel_codegen::tables::{assets::assets, digital_assets::digital_assets};
diesel::allow_tables_to_appear_in_same_query!(digital_assets, assets);
use crate::codegen::diesel_codegen::tables::digital_asset_models::digital_asset_models;
diesel::allow_tables_to_appear_in_same_query!(digital_assets, digital_asset_models);
