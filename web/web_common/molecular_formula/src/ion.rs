//! Submodule providing the `Ion` struct

use std::fmt::Display;

use elements::Element;
use fmtastic::Superscript;

use crate::MolecularFormula;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Ion struct representing an ion with a specific element and charge.
pub struct Ion<E> {
    /// Can be an element or a molecular formula
    pub(crate) entry: E,
    /// Charge
    pub(crate) charge: i16,
}

impl Ion<Element> {
    /// Creates a new `Ion` instance with the given element and charge.
    ///
    /// # Arguments
    ///
    /// * `element` - The element to be used as the entry.
    /// * `charge` - The charge of the ion.
    ///
    /// # Errors
    ///
    /// * If the charge is 0 or not in the oxidation states of the element, an
    ///   error is returned.
    /// * If the charge is not in the oxidation states of the element, an error
    ///   is returned.
    pub fn from_element(element: Element, charge: i16) -> Result<Self, crate::errors::Error> {
        if charge == 0 {
            return Err(crate::errors::Error::ZeroCharge);
        }
        if !element.oxidation_states().contains(&charge) {
            return Err(crate::errors::Error::InvalidOxidationState(charge));
        }
        Ok(Ion { entry: element, charge })
    }
}

impl Ion<MolecularFormula> {
    /// Creates a new `Ion` instance with the given formula and charge.
    ///
    /// # Arguments
    ///
    /// * `formula` - The formula to be used as the entry.
    /// * `charge` - The charge of the ion.
    ///
    /// # Errors
    ///
    /// * If the charge is 0 or not in the oxidation states of the element, an
    ///   error is returned.
    pub fn from_formula(
        formula: MolecularFormula,
        charge: i16,
    ) -> Result<Self, crate::errors::Error> {
        if charge == 0 {
            return Err(crate::errors::Error::ZeroCharge);
        }
        if !formula.is_valid_oxidation_state(charge)? {
            return Err(crate::errors::Error::InvalidOxidationState(charge));
        }

        Ok(Ion { entry: formula, charge })
    }
}

impl From<Ion<elements::Element>> for Ion<MolecularFormula> {
    fn from(ion: Ion<elements::Element>) -> Self {
        Ion { entry: MolecularFormula::Element(ion.entry), charge: ion.charge }
    }
}

impl From<Ion<elements::Element>> for Ion<Box<MolecularFormula>> {
    fn from(ion: Ion<elements::Element>) -> Self {
        Ion { entry: Box::new(MolecularFormula::Element(ion.entry)), charge: ion.charge }
    }
}

impl From<Ion<MolecularFormula>> for Ion<Box<MolecularFormula>> {
    fn from(ion: Ion<MolecularFormula>) -> Self {
        Ion { entry: Box::new(ion.entry), charge: ion.charge }
    }
}

/// Electron mass in amu
pub const ELECTRON_MASS: f64 = 0.000_548_579_91;

impl<E> Display for Ion<E>
where
    E: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.charge == -1 {
            write!(f, "{}⁻", self.entry)
        } else if self.charge == 1 {
            write!(f, "{}⁺", self.entry)
        } else if self.charge < 0 {
            write!(f, "{}{}", self.entry, Superscript(self.charge))
        } else {
            write!(f, "{}⁺{}", self.entry, Superscript(self.charge))
        }
    }
}
