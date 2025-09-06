diesel::table! {
    centrifuge_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, centrifuged_container ->
    ::rosetta_uuid::diesel_impls::Uuid, centrifuged_container_model ->
    diesel::sql_types::Integer, procedure_template_centrifuged_container_model ->
    diesel::sql_types::Integer, procedure_centrifuged_container ->
    ::rosetta_uuid::diesel_impls::Uuid, centrifuged_with_model ->
    diesel::sql_types::Integer, centrifuged_with -> diesel::sql_types::Nullable <
    ::rosetta_uuid::diesel_impls::Uuid >, procedure_template_centrifuged_with_model ->
    diesel::sql_types::Integer, procedure_centrifuged_with ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
