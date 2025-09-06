diesel::table! {
    ball_mill_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, bead_model ->
    diesel::sql_types::Integer, procedure_template_bead_model ->
    diesel::sql_types::Integer, procedure_bead -> ::rosetta_uuid::diesel_impls::Uuid,
    milled_with_model -> diesel::sql_types::Integer, procedure_template_milled_with_model
    -> diesel::sql_types::Integer, procedure_milled_with ->
    ::rosetta_uuid::diesel_impls::Uuid, milled_with -> diesel::sql_types::Nullable <
    ::rosetta_uuid::diesel_impls::Uuid >, milled_container ->
    ::rosetta_uuid::diesel_impls::Uuid, milled_container_model ->
    diesel::sql_types::Integer, procedure_template_milled_container_model ->
    diesel::sql_types::Integer, procedure_milled_container ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
