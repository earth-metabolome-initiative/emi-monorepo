diesel::table! {
    fractioning_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, fragment_container ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_template_fragment_container_model ->
    diesel::sql_types::Integer, procedure_fragment_container ->
    ::rosetta_uuid::diesel_impls::Uuid, fragment_placed_into ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_template_fragment_placed_into_model ->
    diesel::sql_types::Integer, procedure_fragment_placed_into ->
    ::rosetta_uuid::diesel_impls::Uuid, kilograms -> diesel::sql_types::Float,
    weighed_with -> diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >,
    procedure_template_weighed_with_model -> diesel::sql_types::Integer,
    procedure_weighed_with -> ::rosetta_uuid::diesel_impls::Uuid }
}
