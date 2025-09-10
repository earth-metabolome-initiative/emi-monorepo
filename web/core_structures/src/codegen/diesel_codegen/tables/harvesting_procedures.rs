diesel::table! {
    harvesting_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, sample_source ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_template_sample_source_model ->
    diesel::sql_types::Integer, procedure_sample_source ->
    ::rosetta_uuid::diesel_impls::Uuid, sample -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template_sample_model -> diesel::sql_types::Integer, procedure_sample ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
