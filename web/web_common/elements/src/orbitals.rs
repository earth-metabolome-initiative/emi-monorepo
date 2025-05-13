//! Submodule providing the `AtomicOrbital` enumeration and the method to obtain
//! the orbitals for a given element.

mod element;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Enum representing the types of orbitals.
pub enum AtomicOrbitalType {
    /// s orbital
    S,
    /// p orbital
    P,
    /// d orbital
    D,
    /// f orbital
    F,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing an orbital.
pub struct AtomicOrbital {
    /// The principal quantum number of the orbital
    principal_quantum_number: u8,
    /// The type of the orbital as defined by the azimuthal quantum number
    orbital_type: AtomicOrbitalType,
    /// The number of electrons in the orbital
    number_of_electrons: u8,
}

impl AtomicOrbital {
    const fn new(
        principal_quantum_number: u8,
        orbital_type: AtomicOrbitalType,
        number_of_electrons: u8,
    ) -> Self {
        Self { principal_quantum_number, orbital_type, number_of_electrons }
    }
    #[must_use]
    /// Returns the orbitals for a given element.
    pub fn principal_quantum_number(&self) -> u8 {
        self.principal_quantum_number
    }
    #[must_use]
    /// Returns the type of the orbital.
    pub fn orbital_type(&self) -> AtomicOrbitalType {
        self.orbital_type
    }

    #[must_use]
    /// Returns the number of electrons in the orbital.
    pub fn number_of_electrons(&self) -> u8 {
        self.number_of_electrons
    }
}
