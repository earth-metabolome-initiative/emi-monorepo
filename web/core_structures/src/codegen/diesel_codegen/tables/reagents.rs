diesel::table! {
    reagents(id) { id -> diesel::sql_types::Integer, purity -> diesel::sql_types::Float,
    cas_code -> ::cas_codes::diesel_impls::CAS, molecular_formulas ->
    ::molecular_formulas::molecular_formula::diesel_impls::MolecularFormula, created_by
    -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
