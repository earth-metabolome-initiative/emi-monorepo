diesel::table! {
    public.spatial_ref_sys(srid) { srid -> diesel::sql_types::Integer, auth_name ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, auth_srid ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer >, srtext ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, proj4text ->
    diesel::sql_types::Nullable < diesel::sql_types::Text > }
}
