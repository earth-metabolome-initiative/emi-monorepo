use crate::codegen::diesel_codegen::tables::{
    pipette_models::pipette_models, supernatant_procedures::supernatant_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(supernatant_procedures, pipette_models);
use crate::codegen::diesel_codegen::tables::pipette_tip_models::pipette_tip_models;
diesel::allow_tables_to_appear_in_same_query!(supernatant_procedures, pipette_tip_models);
use crate::codegen::diesel_codegen::tables::procedure_trackables::procedure_trackables;
diesel::allow_tables_to_appear_in_same_query!(supernatant_procedures, procedure_trackables);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(supernatant_procedures, procedures);
use crate::codegen::diesel_codegen::tables::supernatant_procedure_models::supernatant_procedure_models;
diesel::allow_tables_to_appear_in_same_query!(supernatant_procedures, supernatant_procedure_models);
use crate::codegen::diesel_codegen::tables::volumetric_container_models::volumetric_container_models;
diesel::allow_tables_to_appear_in_same_query!(supernatant_procedures, volumetric_container_models);
