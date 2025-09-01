use crate::codegen::diesel_codegen::tables::{assets::assets, physical_assets::physical_assets};
diesel::allow_tables_to_appear_in_same_query!(physical_assets, assets);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(physical_assets, physical_asset_models);
