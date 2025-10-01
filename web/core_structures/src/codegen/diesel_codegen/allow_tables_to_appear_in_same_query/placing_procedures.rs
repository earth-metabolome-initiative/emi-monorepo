use crate::codegen::diesel_codegen::tables::{
    geolocation_procedures::geolocation_procedures, placing_procedures::placing_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(placing_procedures, geolocation_procedures);
use crate::codegen::diesel_codegen::tables::placing_procedure_templates::placing_procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(placing_procedures, placing_procedure_templates);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(placing_procedures, procedures);
