use crate::codegen::diesel_codegen::tables::{assets::assets, containers::containers};
diesel::allow_tables_to_appear_in_same_query!(containers, assets);
use crate::codegen::diesel_codegen::tables::container_models::container_models;
diesel::allow_tables_to_appear_in_same_query!(containers, container_models);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(containers, physical_assets);
