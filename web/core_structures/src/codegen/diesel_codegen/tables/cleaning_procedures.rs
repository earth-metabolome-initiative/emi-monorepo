diesel::table! {
    cleaning_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer,
    procedure_template_cleaned_with_model -> diesel::sql_types::Integer,
    procedure_cleaned_with -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template_cleaned_model -> diesel::sql_types::Integer, procedure_cleaned ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
