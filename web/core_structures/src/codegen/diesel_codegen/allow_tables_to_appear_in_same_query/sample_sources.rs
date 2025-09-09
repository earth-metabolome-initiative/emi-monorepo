use crate::codegen::diesel_codegen::tables::{assets::assets, sample_sources::sample_sources};
diesel::allow_tables_to_appear_in_same_query!(sample_sources, assets);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(sample_sources, physical_assets);
use crate::codegen::diesel_codegen::tables::sample_source_models::sample_source_models;
diesel::allow_tables_to_appear_in_same_query!(sample_sources, sample_source_models);
