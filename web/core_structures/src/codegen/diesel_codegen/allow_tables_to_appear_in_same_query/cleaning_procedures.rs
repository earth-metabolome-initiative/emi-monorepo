use crate::codegen::diesel_codegen::tables::{
    cleaning_procedure_templates::cleaning_procedure_templates,
    cleaning_procedures::cleaning_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(cleaning_procedures, cleaning_procedure_templates);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(cleaning_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(cleaning_procedures, procedure_template_asset_models);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(cleaning_procedures, procedures);
