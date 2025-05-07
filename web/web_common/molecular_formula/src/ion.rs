//! Submodule providing the `Ion` struct

use std::fmt::Display;

use elements::MolarMass;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Ion struct representing an ion with a specific element and charge.
pub struct Ion<E> {
    /// Can be an element or a molecular formula
    entry: E,
    /// Charge
    charge: i8,
}

impl<E> Ion<E> {
    /// Creates a new `Ion` instance with the given entry and charge.
    ///
    /// # Panics
    ///
    /// * Panics if the charge is zero.
    pub fn new(entry: E, charge: i8) -> Self {
        assert!(charge != 0, "Ion charge cannot be zero. Use Element or MolecularFormula instead.");
        Ion { entry, charge }
    }
}

/// Electron molar mass in amu
pub const ELECTRON_MOLAR_MASS: f64 = 0.00054858;

impl<E> MolarMass for Ion<E>
where
    E: MolarMass,
{
    fn molar_mass(&self) -> f64 {
        self.entry.molar_mass() - f64::from(self.charge) * ELECTRON_MOLAR_MASS
    }
}

impl<E> Display for Ion<E>
where
    E: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.charge.cmp(&0) {
            std::cmp::Ordering::Less => {
                if self.charge == -1 {
                    write!(f, "{}-", self.entry)
                } else {
                    write!(f, "{}{}-", self.entry, -self.charge)
                }
            }
            std::cmp::Ordering::Equal => write!(f, "{}", self.entry),
            std::cmp::Ordering::Greater => {
                if self.charge == 1 {
                    write!(f, "{}+", self.entry)
                } else {
                    write!(f, "{}{}+", self.entry, self.charge)
                }
            }
        }
    }
}
