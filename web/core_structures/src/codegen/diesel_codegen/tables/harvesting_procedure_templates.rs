diesel::table! {
    harvesting_procedure_templates(procedure_template) { procedure_template ->
    diesel::sql_types::Integer, sample_source_model -> diesel::sql_types::Integer,
    procedure_template_sample_source_model -> diesel::sql_types::Integer, sample_model ->
    diesel::sql_types::Integer, procedure_template_sample_model ->
    diesel::sql_types::Integer }
}
