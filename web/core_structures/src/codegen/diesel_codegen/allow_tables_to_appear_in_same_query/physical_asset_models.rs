use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models, physical_asset_models::physical_asset_models,
};
diesel::allow_tables_to_appear_in_same_query!(physical_asset_models, asset_models);
