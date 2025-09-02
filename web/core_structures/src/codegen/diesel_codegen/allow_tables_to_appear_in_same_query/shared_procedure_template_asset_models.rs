use crate::codegen::diesel_codegen::tables::{
    asset_model_ancestors::asset_model_ancestors,
    shared_procedure_template_asset_models::shared_procedure_template_asset_models,
};
diesel::allow_tables_to_appear_in_same_query!(
    shared_procedure_template_asset_models,
    asset_model_ancestors
);
use crate::codegen::diesel_codegen::tables::asset_models::asset_models;
diesel::allow_tables_to_appear_in_same_query!(shared_procedure_template_asset_models, asset_models);
use crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(
    shared_procedure_template_asset_models,
    parent_procedure_templates
);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    shared_procedure_template_asset_models,
    procedure_template_asset_models
);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(
    shared_procedure_template_asset_models,
    procedure_templates
);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(shared_procedure_template_asset_models, users);
