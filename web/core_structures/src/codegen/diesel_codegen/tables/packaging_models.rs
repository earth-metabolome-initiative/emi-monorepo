diesel::table! {
    packaging_models(id) { id -> diesel::sql_types::Integer, kilograms ->
    diesel::sql_types::Float, created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
