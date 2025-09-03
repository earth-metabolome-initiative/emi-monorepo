use crate::codegen::diesel_codegen::tables::{
    assets::assets, registering_procedures::registering_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(registering_procedures, assets);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(registering_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(registering_procedures, procedures);
use crate::codegen::diesel_codegen::tables::registering_procedure_templates::registering_procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(
    registering_procedures,
    registering_procedure_templates
);
