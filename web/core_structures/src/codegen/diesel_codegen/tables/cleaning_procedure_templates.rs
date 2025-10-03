diesel::table! {
    cleaning_procedure_templates(procedure_template) { procedure_template ->
    diesel::sql_types::Integer, cleaned_with_model -> diesel::sql_types::Integer,
    procedure_template_cleaned_with_model -> diesel::sql_types::Integer, cleaned_model ->
    diesel::sql_types::Integer, procedure_template_cleaned_model ->
    diesel::sql_types::Integer, liters -> diesel::sql_types::Float }
}
