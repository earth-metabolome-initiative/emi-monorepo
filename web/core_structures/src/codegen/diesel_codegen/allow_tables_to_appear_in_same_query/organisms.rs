use crate::codegen::diesel_codegen::tables::{
    organisms::organisms, physical_assets::physical_assets,
};
diesel::allow_tables_to_appear_in_same_query!(organisms, physical_assets);
use crate::codegen::diesel_codegen::tables::assets::assets;
diesel::allow_tables_to_appear_in_same_query!(organisms, assets);
