use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models, procedure_template_asset_models::procedure_template_asset_models,
};
diesel::allow_tables_to_appear_in_same_query!(procedure_template_asset_models, asset_models);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(procedure_template_asset_models, procedure_templates);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(procedure_template_asset_models, users);
