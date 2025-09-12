use crate::codegen::diesel_codegen::tables::{assets::assets, soils::soils};
diesel::allow_tables_to_appear_in_same_query!(soils, assets);
use crate::codegen::diesel_codegen::tables::sample_sources::sample_sources;
diesel::allow_tables_to_appear_in_same_query!(soils, sample_sources);
use crate::codegen::diesel_codegen::tables::soil_models::soil_models;
diesel::allow_tables_to_appear_in_same_query!(soils, soil_models);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(soils, physical_assets);
