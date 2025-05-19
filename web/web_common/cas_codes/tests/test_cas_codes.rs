//! Test submodule to validate the CAS codes.

use cas_codes::CAS;

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
    chemical_formula: String,
    #[serde(rename = "Synonyms")]
    synonyms: String,
    #[serde(rename = "CAS number")]
    cas_number: CAS,
}

#[test]
/// Test to validate the CAS codes.
pub fn test_cas_codes() {
    let path = "cas_codes.tsv";
    let file = std::fs::File::open(path).expect("Failed to open the file");
    let reader = std::io::BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').has_headers(true).from_reader(reader);
    for result in rdr.deserialize() {
        let row: RawRow = result.expect("Failed to deserialize row");
        let cas = CAS::try_from(&row.cas_number).expect("Failed to create CAS instance: {row:?}");
        let cas_str = cas.to_string();
        assert_eq!(cas_str, row.cas_number, "CAS number mismatch: {row:?}");
    }

    let file = std::fs::File::open(path).expect("Failed to open the file");
    let reader = std::io::BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').has_headers(true).from_reader(reader);

    for result in rdr.deserialize() {
        let _row: StructuredRow = result.expect("Failed to deserialize row");
    }
}
