use crate::codegen::diesel_codegen::tables::{
    assets::assets, volumetric_containers::volumetric_containers,
};
diesel::allow_tables_to_appear_in_same_query!(volumetric_containers, assets);
use crate::codegen::diesel_codegen::tables::containers::containers;
diesel::allow_tables_to_appear_in_same_query!(volumetric_containers, containers);
use crate::codegen::diesel_codegen::tables::volumetric_container_models::volumetric_container_models;
diesel::allow_tables_to_appear_in_same_query!(volumetric_containers, volumetric_container_models);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(volumetric_containers, physical_assets);
