diesel::table! {
    packaging_procedure_templates(procedure_template_id) { procedure_template ->
    diesel::sql_types::Integer, packaged_with_model -> diesel::sql_types::Integer,
    procedure_template_packaged_with_model -> diesel::sql_types::Integer, sample_model ->
    diesel::sql_types::Integer, procedure_template_sample_model ->
    diesel::sql_types::Integer }
}
