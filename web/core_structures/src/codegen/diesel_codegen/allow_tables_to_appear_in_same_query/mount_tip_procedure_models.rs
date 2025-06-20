use crate::codegen::diesel_codegen::tables::{
    compatibility_rules::compatibility_rules,
    mount_tip_procedure_models::mount_tip_procedure_models,
};
diesel::allow_tables_to_appear_in_same_query!(mount_tip_procedure_models, compatibility_rules);
use crate::codegen::diesel_codegen::tables::pipette_models::pipette_models;
diesel::allow_tables_to_appear_in_same_query!(mount_tip_procedure_models, pipette_models);
use crate::codegen::diesel_codegen::tables::pipette_tip_models::pipette_tip_models;
diesel::allow_tables_to_appear_in_same_query!(mount_tip_procedure_models, pipette_tip_models);
use crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables;
diesel::allow_tables_to_appear_in_same_query!(
    mount_tip_procedure_models,
    procedure_model_trackables
);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(mount_tip_procedure_models, procedure_models);
