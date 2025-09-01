diesel::table! {
    photograph_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, foreign_procedure_template ->
    diesel::sql_types::Integer, foreign_procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    photographed_asset -> ::rosetta_uuid::diesel_impls::Uuid, photographed_with ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >,
    photographed_with_model -> diesel::sql_types::Integer }
}
