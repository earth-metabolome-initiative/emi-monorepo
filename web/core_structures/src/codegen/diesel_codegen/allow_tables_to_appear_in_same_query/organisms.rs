use crate::codegen::diesel_codegen::tables::{
    organisms::organisms, sample_sources::sample_sources,
};
diesel::allow_tables_to_appear_in_same_query!(organisms, sample_sources);
use crate::codegen::diesel_codegen::tables::assets::assets;
diesel::allow_tables_to_appear_in_same_query!(organisms, assets);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(organisms, physical_assets);
