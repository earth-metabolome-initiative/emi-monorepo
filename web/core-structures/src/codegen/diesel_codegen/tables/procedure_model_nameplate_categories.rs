diesel::table! {
    procedure_model_nameplate_categories(id) { id -> diesel::sql_types::Integer,
    procedure_model_id -> diesel::sql_types::Integer, nameplate_category ->
    nameplate_categories::diesel_impls::NameplateCategory, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
