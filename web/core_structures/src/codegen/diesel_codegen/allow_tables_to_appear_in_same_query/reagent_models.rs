use crate::codegen::diesel_codegen::tables::{
    physical_asset_models::physical_asset_models, reagent_models::reagent_models,
};
diesel::allow_tables_to_appear_in_same_query!(reagent_models, physical_asset_models);
use crate::codegen::diesel_codegen::tables::asset_models::asset_models;
diesel::allow_tables_to_appear_in_same_query!(reagent_models, asset_models);
