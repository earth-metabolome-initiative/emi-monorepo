diesel::table! {
    freezing_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, foreign_procedure_template ->
    diesel::sql_types::Integer, foreign_procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    frozen_container -> ::rosetta_uuid::diesel_impls::Uuid, frozen_with ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, frozen_with_model
    -> diesel::sql_types::Integer }
}
