diesel::table! {
    freezing_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, frozen_container ->
    ::rosetta_uuid::diesel_impls::Uuid, frozen_container_model ->
    diesel::sql_types::Integer, procedure_template_frozen_container_model ->
    diesel::sql_types::Integer, procedure_frozen_container ->
    ::rosetta_uuid::diesel_impls::Uuid, frozen_with -> diesel::sql_types::Nullable <
    ::rosetta_uuid::diesel_impls::Uuid >, frozen_with_model ->
    diesel::sql_types::Integer, procedure_template_frozen_with_model ->
    diesel::sql_types::Integer, procedure_frozen_with ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
