diesel::table! {
    capping_procedure_templates(procedure_template_id) { procedure_template ->
    diesel::sql_types::Integer, capped_container_model -> diesel::sql_types::Integer,
    procedure_template_capped_container_model -> diesel::sql_types::Integer,
    capped_with_model -> diesel::sql_types::Integer, procedure_template_capped_with_model
    -> diesel::sql_types::Integer }
}
