use crate::codegen::diesel_codegen::tables::{
    beads_models::beads_models, physical_asset_models::physical_asset_models,
};
diesel::allow_tables_to_appear_in_same_query!(beads_models, physical_asset_models);
use crate::codegen::diesel_codegen::tables::asset_models::asset_models;
diesel::allow_tables_to_appear_in_same_query!(beads_models, asset_models);
