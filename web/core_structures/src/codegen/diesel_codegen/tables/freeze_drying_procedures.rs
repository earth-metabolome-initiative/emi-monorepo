diesel::table! {
    freeze_drying_procedures(procedure) { procedure ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_template -> diesel::sql_types::Integer,
    freeze_dried_container -> ::rosetta_uuid::diesel_impls::Uuid,
    freeze_dried_container_model -> diesel::sql_types::Integer,
    procedure_template_freeze_dried_container_model -> diesel::sql_types::Integer,
    procedure_freeze_dried_container -> ::rosetta_uuid::diesel_impls::Uuid,
    freeze_dried_with -> diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid
    >, freeze_dried_with_model -> diesel::sql_types::Integer,
    procedure_template_freeze_dried_with_model -> diesel::sql_types::Integer,
    procedure_freeze_dried_with -> ::rosetta_uuid::diesel_impls::Uuid }
}
