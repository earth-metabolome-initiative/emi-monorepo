diesel::table! {
    pouring_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, poured_from ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >,
    procedure_template_poured_from_model -> diesel::sql_types::Integer,
    procedure_poured_from -> ::rosetta_uuid::diesel_impls::Uuid, measured_with ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >,
    procedure_template_measured_with_model -> diesel::sql_types::Integer,
    procedure_measured_with -> ::rosetta_uuid::diesel_impls::Uuid, poured_into ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_template_poured_into_model ->
    diesel::sql_types::Integer, procedure_poured_into ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
