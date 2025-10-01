use crate::codegen::diesel_codegen::tables::{
    geolocation_procedure_templates::geolocation_procedure_templates,
    placing_procedure_templates::placing_procedure_templates,
};
diesel::allow_tables_to_appear_in_same_query!(
    placing_procedure_templates,
    geolocation_procedure_templates
);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(placing_procedure_templates, procedure_templates);
