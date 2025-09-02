use crate::codegen::diesel_codegen::tables::{
    packaging_models::packaging_models, packaging_procedures::packaging_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(packaging_procedures, packaging_models);
use crate::codegen::diesel_codegen::tables::packaging_procedure_templates::packaging_procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(packaging_procedures, packaging_procedure_templates);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(packaging_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(packaging_procedures, procedures);
