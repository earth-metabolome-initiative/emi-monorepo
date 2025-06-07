diesel::table! {
    reagents(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, purity ->
    diesel::sql_types::Float, cas_code -> ::cas_codes::diesel_impls::CAS,
    molecular_formula ->
    ::molecular_formulas::molecular_formula::diesel_impls::MolecularFormula }
}
