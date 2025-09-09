use crate::codegen::diesel_codegen::tables::{
    digital_assets::digital_assets, photographs::photographs,
};
diesel::allow_tables_to_appear_in_same_query!(photographs, digital_assets);
use crate::codegen::diesel_codegen::tables::assets::assets;
diesel::allow_tables_to_appear_in_same_query!(photographs, assets);
