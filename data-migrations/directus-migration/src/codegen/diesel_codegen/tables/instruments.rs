diesel::table! {
    #[sql_name = "Instruments"] public.instruments(id) { id ->
    diesel::sql_types::Integer, status -> diesel::sql_types::Text, user_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, date_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, user_updated ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, date_updated ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, uuid_instrument ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, instrument_id ->
    diesel::sql_types::Text, instrument_location -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer >, instrument_model -> diesel::sql_types::Integer }
}
