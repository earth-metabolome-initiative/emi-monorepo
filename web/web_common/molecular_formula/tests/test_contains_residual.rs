//! Test submodule for `contains_residual` method of `MolecularFormula` struct.

use std::str::FromStr;

use molecular_formula::MolecularFormula;

const FORMULAS_WITH_RESIDUALS: &[&str] = &[
    "CH4R", "C2H6R", "C3H8R", "C4H10R", "C5H12R", "C6H14R", "C7H16R", "C8H18R", "C9H20R", "C10H22R",
];
const FORMULAS_WITHOUT_RESIDUALS: &[&str] =
    &["CH4", "C2H6", "C3H8", "C4H10", "C5H12", "C6H14", "C7H16", "C8H18", "C9H20", "C10H22"];

#[test]
/// Test to check that the `contains_residual` method works as expected
fn test_residual_detection() {
    for formula in FORMULAS_WITH_RESIDUALS {
        let formula =
            MolecularFormula::from_str(formula).expect(&format!("Failed to parse `{}`", formula));
        assert!(formula.contains_residual(), "Expected {} to contain a residual", formula);
        assert!(formula.molar_mass().is_err(), "Expected {} to have no known molar mass", formula);
    }

    for formula in FORMULAS_WITHOUT_RESIDUALS {
        let formula = MolecularFormula::from_str(formula).unwrap();
        assert!(!formula.contains_residual(), "Expected {} not to contain a residual", formula);
    }
}
