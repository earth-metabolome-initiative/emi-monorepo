diesel::table! {
    aliquoting_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, aliquoted_with ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >,
    aliquoted_with_model -> diesel::sql_types::Integer,
    procedure_template_aliquoted_with_model -> diesel::sql_types::Integer,
    procedure_aliquoted_with -> ::rosetta_uuid::diesel_impls::Uuid, pipette_tip_model ->
    diesel::sql_types::Integer, procedure_template_pipette_tip_model ->
    diesel::sql_types::Integer, procedure_pipette_tip ->
    ::rosetta_uuid::diesel_impls::Uuid, aliquoted_from ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_template_aliquoted_from_model ->
    diesel::sql_types::Integer, procedure_aliquoted_from ->
    ::rosetta_uuid::diesel_impls::Uuid, aliquoted_into ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_template_aliquoted_into_model ->
    diesel::sql_types::Integer, procedure_aliquoted_into ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
