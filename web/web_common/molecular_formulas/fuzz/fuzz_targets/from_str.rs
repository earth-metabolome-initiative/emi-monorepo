//! Submodule for fuzzing the execution of the Hopcroft-Karp algorithm.

use honggfuzz::fuzz;
use std::str::FromStr;
use molecular_formulas::MolecularFormula;

fn main() {
    loop {
        fuzz!(|candidate: &str| {
            // If the candidate has more than 10 characters, skip it
            if candidate.len() > 10 {
                return;
            }

            let start_time = std::time::Instant::now();
            let Ok(formula) = MolecularFormula::from_str(candidate) else {
                // If the candidate is not a valid formula, we can skip it
                return;
            };
            let elapsed = start_time.elapsed();

            // If the parsing took more than 0.5 second, we raise an error
            // so to turn a timeout into a panic
            if elapsed.as_secs_f64() > 0.5 {
                panic!("Parsing candidate `{candidate}` took too long: {} seconds", elapsed.as_secs_f64());
            }

            // We check that the display works without panicking
            let _ = formula.to_string();

            let contains_residual = formula.contains_residual();
        });
    }
}
