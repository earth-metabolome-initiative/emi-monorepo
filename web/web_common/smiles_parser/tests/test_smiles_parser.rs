//! Tests of the parser module for several corner cases.

use smiles_parser::prelude::Smiles;
const SMILES_STR: &[&str] = &["C1=CC=CC=C1"]; // Benzene

#[test]
fn test_parser() {
    for &s in SMILES_STR {
        let _smiles: Smiles = s.parse().unwrap();
    }
}
