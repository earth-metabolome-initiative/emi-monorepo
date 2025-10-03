use crate::codegen::diesel_codegen::tables::{
    personal_protective_equipment_models::personal_protective_equipment_models,
    ppe_reminder_procedure_templates::ppe_reminder_procedure_templates,
};
diesel::allow_tables_to_appear_in_same_query!(
    ppe_reminder_procedure_templates,
    personal_protective_equipment_models
);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    ppe_reminder_procedure_templates,
    procedure_template_asset_models
);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(
    ppe_reminder_procedure_templates,
    procedure_templates
);
