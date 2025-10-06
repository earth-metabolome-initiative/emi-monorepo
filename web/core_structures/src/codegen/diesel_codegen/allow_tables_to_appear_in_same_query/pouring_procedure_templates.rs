use crate::codegen::diesel_codegen::tables::{
    physical_asset_models::physical_asset_models,
    pouring_procedure_templates::pouring_procedure_templates,
};
diesel::allow_tables_to_appear_in_same_query!(pouring_procedure_templates, physical_asset_models);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    pouring_procedure_templates,
    procedure_template_asset_models
);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(pouring_procedure_templates, procedure_templates);
use crate::codegen::diesel_codegen::tables::volume_measuring_device_models::volume_measuring_device_models;
diesel::allow_tables_to_appear_in_same_query!(
    pouring_procedure_templates,
    volume_measuring_device_models
);
use crate::codegen::diesel_codegen::tables::volumetric_container_models::volumetric_container_models;
diesel::allow_tables_to_appear_in_same_query!(
    pouring_procedure_templates,
    volumetric_container_models
);
