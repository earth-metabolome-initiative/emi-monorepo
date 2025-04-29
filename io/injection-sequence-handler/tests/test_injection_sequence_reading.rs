//! Test module for reading injection sequences from files.
//! This module contains tests for reading injection sequences from files.
//! It includes tests for reading Xcalibur injection sequences and handling errors.

use injection_sequence_handler::structs::xcalibur::SparseRow;


#[test]
/// Test for reading Xcalibur injection sequences
fn test_xcalibur_sequence_reading() {
    let mut reader = csv::Reader::from_path("./data/Template_Sample_List_XCalibur_noprefix.csv").expect("Unable to read csv sequence file !");

    let rows = reader.deserialize().collect::<Result<Vec<SparseRow>, _>>().expect("Unable to deserialize rows !");

    assert_eq!(rows.len(), 8);
    

}