use crate::codegen::diesel_codegen::tables::{assets::assets, organisms::organisms};
diesel::allow_tables_to_appear_in_same_query!(organisms, assets);
use crate::codegen::diesel_codegen::tables::organism_models::organism_models;
diesel::allow_tables_to_appear_in_same_query!(organisms, organism_models);
use crate::codegen::diesel_codegen::tables::sample_sources::sample_sources;
diesel::allow_tables_to_appear_in_same_query!(organisms, sample_sources);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(organisms, physical_assets);
