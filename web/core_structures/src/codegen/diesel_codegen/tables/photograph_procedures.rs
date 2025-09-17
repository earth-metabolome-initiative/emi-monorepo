diesel::table! {
    photograph_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, photographed_asset ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >,
    procedure_template_photographed_asset_model -> diesel::sql_types::Integer,
    procedure_photographed_asset -> ::rosetta_uuid::diesel_impls::Uuid, photographed_with
    -> diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >,
    procedure_template_photographed_with_model -> diesel::sql_types::Integer,
    procedure_photographed_with -> ::rosetta_uuid::diesel_impls::Uuid, photograph ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_template_photograph_model ->
    diesel::sql_types::Integer, procedure_photograph ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
