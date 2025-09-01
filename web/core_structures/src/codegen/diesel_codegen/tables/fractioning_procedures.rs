diesel::table! {
    fractioning_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, foreign_procedure_template ->
    diesel::sql_types::Integer, foreign_procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    fragment_container -> ::rosetta_uuid::diesel_impls::Uuid, fragment_placed_into ->
    ::rosetta_uuid::diesel_impls::Uuid, kilograms -> diesel::sql_types::Float,
    weighed_with -> diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >,
    weighed_with_model -> diesel::sql_types::Integer }
}
