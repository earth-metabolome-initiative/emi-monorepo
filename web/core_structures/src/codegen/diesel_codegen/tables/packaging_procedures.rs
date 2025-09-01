diesel::table! {
    packaging_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, packaged_with_model ->
    diesel::sql_types::Integer }
}
