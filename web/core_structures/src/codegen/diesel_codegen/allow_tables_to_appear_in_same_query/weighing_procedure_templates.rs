use crate::codegen::diesel_codegen::tables::{
    procedure_template_asset_models::procedure_template_asset_models,
    weighing_procedure_templates::weighing_procedure_templates,
};
diesel::allow_tables_to_appear_in_same_query!(
    weighing_procedure_templates,
    procedure_template_asset_models
);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(weighing_procedure_templates, procedure_templates);
use crate::codegen::diesel_codegen::tables::volumetric_container_models::volumetric_container_models;
diesel::allow_tables_to_appear_in_same_query!(
    weighing_procedure_templates,
    volumetric_container_models
);
use crate::codegen::diesel_codegen::tables::weighing_device_models::weighing_device_models;
diesel::allow_tables_to_appear_in_same_query!(weighing_procedure_templates, weighing_device_models);
