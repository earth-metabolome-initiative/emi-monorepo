diesel::table! {
    weighing_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, weighed_container ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_template_weighed_container_model ->
    diesel::sql_types::Integer, procedure_weighed_container ->
    ::rosetta_uuid::diesel_impls::Uuid, kilograms -> diesel::sql_types::Float,
    weighed_with -> diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >,
    procedure_template_weighed_with_model -> diesel::sql_types::Integer,
    procedure_weighed_with -> ::rosetta_uuid::diesel_impls::Uuid }
}
