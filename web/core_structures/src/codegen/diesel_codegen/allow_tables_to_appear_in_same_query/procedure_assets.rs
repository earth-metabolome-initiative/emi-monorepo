use crate::codegen::diesel_codegen::tables::{
    asset_model_ancestors::asset_model_ancestors, procedure_asset_models::procedure_asset_models,
};
diesel::allow_tables_to_appear_in_same_query!(procedure_asset_models, asset_model_ancestors);
use crate::codegen::diesel_codegen::tables::asset_models::asset_models;
diesel::allow_tables_to_appear_in_same_query!(procedure_asset_models, asset_models);
use crate::codegen::diesel_codegen::tables::assets::assets;
diesel::allow_tables_to_appear_in_same_query!(procedure_asset_models, assets);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(procedure_asset_models, procedure_template_asset_models);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(procedure_asset_models, procedure_templates);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(procedure_asset_models, procedures);
