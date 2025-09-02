use crate::codegen::diesel_codegen::tables::{
    pipette_tip_models::pipette_tip_models, supernatant_procedures::supernatant_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(supernatant_procedures, pipette_tip_models);
use crate::codegen::diesel_codegen::tables::pipettes::pipettes;
diesel::allow_tables_to_appear_in_same_query!(supernatant_procedures, pipettes);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(supernatant_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(supernatant_procedures, procedure_templates);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(supernatant_procedures, procedures);
use crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(
    supernatant_procedures,
    supernatant_procedure_templates
);
use crate::codegen::diesel_codegen::tables::volumetric_containers::volumetric_containers;
diesel::allow_tables_to_appear_in_same_query!(supernatant_procedures, volumetric_containers);
