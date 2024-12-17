use sparql_client::run_sparql_query;
use std::fs;
use std::path::Path;
use std::io::Write;

const TEST_QUERY_FILE: &str = "tests/test_query.rq";
const EXPECTED_CSV_FILE: &str = "tests/expected_results.csv";
const OUTPUT_CSV_FILE: &str = "tests/output_results.csv";
const TEST_ENDPOINT: &str = "https://qlever.cs.uni-freiburg.de/api/wikidata";

#[test]
fn test_run_sparql_query_success() {
    let endpoint = "https://qlever.cs.uni-freiburg.de/api/wikidata";
    let query_file = "tests/data/test_query.rq";
    let output_file = "tests/data/output.csv";

    // Ensure previous output is removed
    if std::path::Path::new(output_file).exists() {
        std::fs::remove_file(output_file).unwrap();
    }

    // Run the SPARQL query
    assert!(
        run_sparql_query(endpoint, query_file, output_file).is_ok(),
        "The SPARQL query execution should succeed."
    );

    // Check that the output file exists
    assert!(
        std::path::Path::new(output_file).exists(),
        "The output CSV file should exist."
    );

    // Check the contents of the output file
    let expected_output = std::fs::read_to_string("tests/data/expected_results.csv").unwrap();
    let actual_output = std::fs::read_to_string(output_file).unwrap();

    assert_eq!(
        expected_output, actual_output,
        "The output CSV file does not match the expected results."
    );
}

#[test]
fn test_non_existent_query_file() {
    let invalid_query_file = "tests/non_existent_query.rq";
    let result = run_sparql_query(TEST_ENDPOINT, invalid_query_file, OUTPUT_CSV_FILE);

    assert!(
        result.is_err(),
        "Running a SPARQL query with a non-existent file should fail."
    );
}

#[test]
fn test_empty_query_file() {
    let empty_query_file = "tests/empty_query.rq";

    // Create an empty file
    let mut file = fs::File::create(empty_query_file).expect("Failed to create empty query file");
    writeln!(file, "").expect("Failed to write empty query file");

    let result = run_sparql_query(TEST_ENDPOINT, empty_query_file, OUTPUT_CSV_FILE);

    assert!(
        result.is_err(),
        "Running a SPARQL query with an empty query file should fail."
    );

    // Clean up
    fs::remove_file(empty_query_file).expect("Failed to clean up empty query file.");
}

#[test]
fn test_output_file_write_error() {
    let invalid_output_file = "/invalid/path/output.csv";

    let result = run_sparql_query(TEST_ENDPOINT, TEST_QUERY_FILE, invalid_output_file);

    assert!(
        result.is_err(),
        "Running a SPARQL query with an invalid output file path should fail."
    );
}
