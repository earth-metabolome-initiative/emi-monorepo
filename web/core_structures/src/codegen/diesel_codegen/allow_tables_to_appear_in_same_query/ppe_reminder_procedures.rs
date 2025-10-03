use crate::codegen::diesel_codegen::tables::{
    ppe_reminder_procedure_templates::ppe_reminder_procedure_templates,
    ppe_reminder_procedures::ppe_reminder_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(
    ppe_reminder_procedures,
    ppe_reminder_procedure_templates
);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(ppe_reminder_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    ppe_reminder_procedures,
    procedure_template_asset_models
);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(ppe_reminder_procedures, procedures);
