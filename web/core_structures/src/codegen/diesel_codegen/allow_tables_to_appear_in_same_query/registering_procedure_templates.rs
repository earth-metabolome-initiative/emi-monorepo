use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models, registering_procedure_templates::registering_procedure_templates,
};
diesel::allow_tables_to_appear_in_same_query!(registering_procedure_templates, asset_models);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    registering_procedure_templates,
    procedure_template_asset_models
);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(registering_procedure_templates, procedure_templates);
