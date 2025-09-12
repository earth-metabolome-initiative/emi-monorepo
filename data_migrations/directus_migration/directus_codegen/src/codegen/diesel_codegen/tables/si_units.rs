diesel::table! {
    #[sql_name = "SI_Units"] si_units(id) { id -> diesel::sql_types::Integer, status ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, user_created ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, date_created ->
    diesel::sql_types::Nullable < rosetta_timestamp::diesel_impls::TimestampUTC >,
    user_updated -> diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >,
    date_updated -> diesel::sql_types::Nullable <
    rosetta_timestamp::diesel_impls::TimestampUTC >, unit_name ->
    diesel::sql_types::Text, symbol -> diesel::sql_types::Text, base_unit ->
    diesel::sql_types::Text, multiplication_factor -> diesel::sql_types::Float }
}
