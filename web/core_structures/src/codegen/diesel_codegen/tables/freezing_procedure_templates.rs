diesel::table! {
    freezing_procedure_templates(procedure_template) { procedure_template ->
    diesel::sql_types::Integer, kelvin -> diesel::sql_types::Float,
    kelvin_tolerance_percentage -> diesel::sql_types::Float, seconds ->
    diesel::sql_types::Nullable < diesel::sql_types::Float >, frozen_with_model ->
    diesel::sql_types::Integer, procedure_template_frozen_with_model ->
    diesel::sql_types::Integer, frozen_container_model -> diesel::sql_types::Integer,
    foreign_procedure_template -> diesel::sql_types::Integer,
    procedure_template_frozen_container_model -> diesel::sql_types::Integer }
}
