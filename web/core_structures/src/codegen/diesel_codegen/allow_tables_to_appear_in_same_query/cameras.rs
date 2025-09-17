use crate::codegen::diesel_codegen::tables::{assets::assets, cameras::cameras};
diesel::allow_tables_to_appear_in_same_query!(cameras, assets);
use crate::codegen::diesel_codegen::tables::commercial_camera_lots::commercial_camera_lots;
diesel::allow_tables_to_appear_in_same_query!(cameras, commercial_camera_lots);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(cameras, physical_assets);
