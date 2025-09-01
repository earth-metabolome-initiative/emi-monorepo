use crate::codegen::diesel_codegen::tables::{
    freeze_dryer_models::freeze_dryer_models, physical_asset_models::physical_asset_models,
};
diesel::allow_tables_to_appear_in_same_query!(freeze_dryer_models, physical_asset_models);
use crate::codegen::diesel_codegen::tables::asset_models::asset_models;
diesel::allow_tables_to_appear_in_same_query!(freeze_dryer_models, asset_models);
