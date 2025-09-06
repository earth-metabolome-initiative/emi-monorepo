use crate::codegen::diesel_codegen::tables::{
    asset_compatibility_rules::asset_compatibility_rules,
    packaging_procedures::packaging_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(packaging_procedures, asset_compatibility_rules);
use crate::codegen::diesel_codegen::tables::packaging_models::packaging_models;
diesel::allow_tables_to_appear_in_same_query!(packaging_procedures, packaging_models);
use crate::codegen::diesel_codegen::tables::packaging_procedure_templates::packaging_procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(packaging_procedures, packaging_procedure_templates);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(packaging_procedures, physical_asset_models);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(packaging_procedures, physical_assets);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(packaging_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    packaging_procedures,
    procedure_template_asset_models
);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(packaging_procedures, procedures);
