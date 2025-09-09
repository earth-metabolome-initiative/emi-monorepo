use crate::codegen::diesel_codegen::tables::{
    camera_models::camera_models, photograph_procedure_templates::photograph_procedure_templates,
};
diesel::allow_tables_to_appear_in_same_query!(photograph_procedure_templates, camera_models);
use crate::codegen::diesel_codegen::tables::digital_asset_models::digital_asset_models;
diesel::allow_tables_to_appear_in_same_query!(photograph_procedure_templates, digital_asset_models);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    photograph_procedure_templates,
    physical_asset_models
);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    photograph_procedure_templates,
    procedure_template_asset_models
);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(photograph_procedure_templates, procedure_templates);
