use crate::codegen::diesel_codegen::tables::{
    organism_models::organism_models, sample_source_models::sample_source_models,
};
diesel::allow_tables_to_appear_in_same_query!(organism_models, sample_source_models);
use crate::codegen::diesel_codegen::tables::asset_models::asset_models;
diesel::allow_tables_to_appear_in_same_query!(organism_models, asset_models);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(organism_models, physical_asset_models);
