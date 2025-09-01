use crate::codegen::diesel_codegen::tables::{
    ball_mill_procedure_templates::ball_mill_procedure_templates,
    procedure_templates::procedure_templates,
};
diesel::allow_tables_to_appear_in_same_query!(ball_mill_procedure_templates, procedure_templates);
use crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules;
diesel::allow_tables_to_appear_in_same_query!(
    ball_mill_procedure_templates,
    asset_compatibility_rules
);
use crate::codegen::diesel_codegen::tables::ball_mill_machine_models::ball_mill_machine_models;
diesel::allow_tables_to_appear_in_same_query!(
    ball_mill_procedure_templates,
    ball_mill_machine_models
);
use crate::codegen::diesel_codegen::tables::beads_models::beads_models;
diesel::allow_tables_to_appear_in_same_query!(ball_mill_procedure_templates, beads_models);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    ball_mill_procedure_templates,
    procedure_template_asset_models
);
use crate::codegen::diesel_codegen::tables::volumetric_container_models::volumetric_container_models;
diesel::allow_tables_to_appear_in_same_query!(
    ball_mill_procedure_templates,
    volumetric_container_models
);
