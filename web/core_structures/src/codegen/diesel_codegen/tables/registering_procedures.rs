diesel::table! {
    registering_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, registered_asset ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
