use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models, digital_asset_models::digital_asset_models,
};
diesel::allow_tables_to_appear_in_same_query!(digital_asset_models, asset_models);
