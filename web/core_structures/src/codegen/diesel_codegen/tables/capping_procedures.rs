diesel::table! {
    capping_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, capped_container ->
    ::rosetta_uuid::diesel_impls::Uuid, capped_container_model ->
    diesel::sql_types::Integer, procedure_template_capped_container_model ->
    diesel::sql_types::Integer, procedure_capped_container ->
    ::rosetta_uuid::diesel_impls::Uuid, capped_with_model -> diesel::sql_types::Integer,
    procedure_template_capped_with_model -> diesel::sql_types::Integer,
    procedure_capped_with -> ::rosetta_uuid::diesel_impls::Uuid }
}
