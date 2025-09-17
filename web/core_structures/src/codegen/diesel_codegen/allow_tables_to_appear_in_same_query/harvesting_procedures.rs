use crate::codegen::diesel_codegen::tables::{
    harvesting_procedure_templates::harvesting_procedure_templates,
    harvesting_procedures::harvesting_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(
    harvesting_procedures,
    harvesting_procedure_templates
);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(harvesting_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    harvesting_procedures,
    procedure_template_asset_models
);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(harvesting_procedures, procedures);
use crate::codegen::diesel_codegen::tables::sample_sources::sample_sources;
diesel::allow_tables_to_appear_in_same_query!(harvesting_procedures, sample_sources);
use crate::codegen::diesel_codegen::tables::samples::samples;
diesel::allow_tables_to_appear_in_same_query!(harvesting_procedures, samples);
