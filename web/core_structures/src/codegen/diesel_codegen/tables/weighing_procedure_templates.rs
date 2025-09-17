diesel::table! {
    weighing_procedure_templates(procedure_template) { procedure_template ->
    diesel::sql_types::Integer, weighed_container_model -> diesel::sql_types::Integer,
    procedure_template_weighed_container_model -> diesel::sql_types::Integer,
    weighed_with_model -> diesel::sql_types::Integer,
    procedure_template_weighed_with_model -> diesel::sql_types::Integer }
}
