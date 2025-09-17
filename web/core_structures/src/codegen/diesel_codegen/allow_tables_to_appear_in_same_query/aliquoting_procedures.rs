use crate::codegen::diesel_codegen::tables::{
    aliquoting_procedure_templates::aliquoting_procedure_templates,
    aliquoting_procedures::aliquoting_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(
    aliquoting_procedures,
    aliquoting_procedure_templates
);
use crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_procedures, asset_compatibility_rules);
use crate::codegen::diesel_codegen::tables::pipette_models::pipette_models;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_procedures, pipette_models);
use crate::codegen::diesel_codegen::tables::pipette_tip_models::pipette_tip_models;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_procedures, pipette_tip_models);
use crate::codegen::diesel_codegen::tables::pipettes::pipettes;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_procedures, pipettes);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    aliquoting_procedures,
    procedure_template_asset_models
);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_procedures, procedures);
use crate::codegen::diesel_codegen::tables::volumetric_containers::volumetric_containers;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_procedures, volumetric_containers);
