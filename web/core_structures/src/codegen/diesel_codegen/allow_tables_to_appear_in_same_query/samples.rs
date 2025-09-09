use crate::codegen::diesel_codegen::tables::{assets::assets, samples::samples};
diesel::allow_tables_to_appear_in_same_query!(samples, assets);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(samples, physical_assets);
use crate::codegen::diesel_codegen::tables::sample_models::sample_models;
diesel::allow_tables_to_appear_in_same_query!(samples, sample_models);
use crate::codegen::diesel_codegen::tables::sample_sources::sample_sources;
diesel::allow_tables_to_appear_in_same_query!(samples, sample_sources);
