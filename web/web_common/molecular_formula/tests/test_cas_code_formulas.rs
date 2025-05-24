//! Test submodule to validate the CAS codes.

use molecular_formula::MolecularFormula;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
struct RawRow {
    #[serde(rename = "Chemical formula")]
    chemical_formula: String,
    #[serde(rename = "Synonyms")]
    synonyms: String,
    #[serde(rename = "CAS number")]
    cas_number: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
struct StructuredRow {
    #[serde(rename = "Chemical formula")]
    chemical_formula: MolecularFormula,
    #[serde(rename = "Synonyms")]
    synonyms: String,
    #[serde(rename = "CAS number")]
    cas_number: String,
}

#[test]
/// Test to validate the CAS codes.
pub fn test_cas_codes() {
    let path = "../cas_codes/cas_codes.tsv";
    let file = std::fs::File::open(path).expect("Failed to open the file");
    let reader = std::io::BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').has_headers(true).from_reader(reader);
    for result in rdr.deserialize() {
        let row: RawRow = result.expect("Failed to deserialize row");
        MolecularFormula::try_from(&row.chemical_formula)
            .expect("Failed to create Molecular Formula instance: {row:?}");
    }

    let file = std::fs::File::open(path).expect("Failed to open the file");
    let reader = std::io::BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').has_headers(true).from_reader(reader);

    for result in rdr.deserialize() {
        let _row: StructuredRow = result.expect("Failed to deserialize row");
    }
}
