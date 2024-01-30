use sirius::prelude::*;
use std::path::Path;
// use std::fs;

// #[test]
// fn test_run_sirius() -> Result<(), String> {
//     let sirius = SiriusBuilder::default()
//         .maximal_mz(802.2)?
//         .formula_search_db(FormulaSearchDB::Bio)?
//         .isotope_settings_filter(false)?
//         .enable_canopus()?
//         .build();

//     let input_file_path = Path::new("tests/data/input_sirius.mgf");
//     let output_file_path = Path::new("tests/data/output_sirius");

//     // Check if the file exists before attempting to remove it
//     // if output_file_path.exists() {
//     //     fs::remove_file(output_file_path)
//     //         .map_err(|e| format!("Failed to remove output file: {}", e))?;
//     // }

//     sirius.run(input_file_path, output_file_path).unwrap();

//     Ok(())
// }
