use crate::codegen::diesel_codegen::tables::{
    aliquoting_procedure_templates::aliquoting_procedure_templates,
    procedure_templates::procedure_templates,
};
diesel::allow_tables_to_appear_in_same_query!(aliquoting_procedure_templates, procedure_templates);
use crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules;
diesel::allow_tables_to_appear_in_same_query!(
    aliquoting_procedure_templates,
    asset_compatibility_rules
);
use crate::codegen::diesel_codegen::tables::pipette_models::pipette_models;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_procedure_templates, pipette_models);
use crate::codegen::diesel_codegen::tables::pipette_tip_models::pipette_tip_models;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_procedure_templates, pipette_tip_models);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    aliquoting_procedure_templates,
    procedure_template_asset_models
);
use crate::codegen::diesel_codegen::tables::volumetric_container_models::volumetric_container_models;
diesel::allow_tables_to_appear_in_same_query!(
    aliquoting_procedure_templates,
    volumetric_container_models
);
