diesel::table! {
    packaging_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, sample ->
    ::rosetta_uuid::diesel_impls::Uuid, sample_model -> diesel::sql_types::Integer,
    procedure_template_sample_model -> diesel::sql_types::Integer, procedure_sample ->
    ::rosetta_uuid::diesel_impls::Uuid, packaged_with_model ->
    diesel::sql_types::Integer, procedure_template_packaged_with_model ->
    diesel::sql_types::Integer, procedure_packaged_with ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
