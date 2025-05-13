//! Submodule implementing the `orbitals` method for the `Element` enum.

use super::{AtomicOrbital, AtomicOrbitalType};

impl crate::Element {
    #[must_use]
    #[allow(clippy::too_many_lines)]
    /// Returns the orbitals associated to the element.
    pub fn orbitals(&self) -> Vec<AtomicOrbital> {
        match self {
            Self::H => vec![AtomicOrbital::new(1, AtomicOrbitalType::S, 1)],
            Self::He => vec![AtomicOrbital::new(1, AtomicOrbitalType::S, 2)],
            Self::Li => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 1),
                ]
            }
            Self::Be => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                ]
            }
            Self::B => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 1),
                ]
            }
            Self::C => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 2),
                ]
            }
            Self::N => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 3),
                ]
            }
            Self::O => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 4),
                ]
            }
            Self::F => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 5),
                ]
            }
            Self::Ne => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                ]
            }
            Self::Na => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 1),
                ]
            }
            Self::Mg => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Al => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 1),
                ]
            }
            Self::Si => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 2),
                ]
            }
            Self::P => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 3),
                ]
            }
            Self::S => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 4),
                ]
            }
            Self::Cl => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 5),
                ]
            }
            Self::Ar => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                ]
            }
            Self::K => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 1),
                ]
            }
            Self::Ca => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Sc => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 1),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Ti => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                ]
            }
            Self::V => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 3),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Cr => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 5),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 1),
                ]
            }
            Self::Mn => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 5),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Fe => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Co => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 7),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Ni => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 8),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Cu => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 1),
                ]
            }
            Self::Zn => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Ga => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 1),
                ]
            }
            Self::Ge => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 2),
                ]
            }
            Self::As => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 3),
                ]
            }
            Self::Se => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 4),
                ]
            }
            Self::Br => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 5),
                ]
            }
            Self::Kr => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                ]
            }
            Self::Rb => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 1),
                ]
            }
            Self::Sr => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Y => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 1),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Zr => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Nb => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 4),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 1),
                ]
            }
            Self::Mo => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 5),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 1),
                ]
            }
            Self::Tc => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 5),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Ru => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 7),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 1),
                ]
            }
            Self::Rh => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 8),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 1),
                ]
            }
            Self::Pd => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                ]
            }
            Self::Ag => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 1),
                ]
            }
            Self::Cd => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                ]
            }
            Self::In => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 1),
                ]
            }
            Self::Sn => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 2),
                ]
            }
            Self::Sb => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 3),
                ]
            }
            Self::Te => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 4),
                ]
            }
            Self::I => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 5),
                ]
            }
            Self::Xe => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                ]
            }
            Self::Cs => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 1),
                ]
            }
            Self::Ba => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::La => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 1),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Ce => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 1),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 1),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Pr => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 3),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Nd => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 4),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Pm => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 5),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Sm => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Eu => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 7),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Gd => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 7),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 1),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Tb => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 9),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Dy => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Ho => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 11),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Er => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 12),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Tm => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 13),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Yb => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Lu => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 1),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Hf => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Ta => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 3),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::W => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 4),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Re => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 5),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Os => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Ir => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 7),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Pt => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 9),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 1),
                ]
            }
            Self::Au => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 1),
                ]
            }
            Self::Hg => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Tl => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 1),
                ]
            }
            Self::Pb => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 2),
                ]
            }
            Self::Bi => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 3),
                ]
            }
            Self::Po => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 4),
                ]
            }
            Self::At => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 5),
                ]
            }
            Self::Rn => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                ]
            }
            Self::Fr => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 1),
                ]
            }
            Self::Ra => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Ac => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 1),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Th => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 2),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Pa => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 1),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::U => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 3),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 1),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Np => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 4),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 1),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Pu => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Am => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 7),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Cm => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 7),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 1),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Bk => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 9),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Cf => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 10),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Es => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 11),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Fm => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 12),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Md => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 13),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::No => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Lr => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(7, AtomicOrbitalType::P, 1),
                ]
            }
            Self::Rf => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 2),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Db => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 3),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Sg => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 4),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Bh => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 5),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Hs => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 6),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Mt => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 7),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Ds => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 8),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Rg => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 9),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Cn => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                ]
            }
            Self::Nh => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(7, AtomicOrbitalType::P, 1),
                ]
            }
            Self::Fl => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(7, AtomicOrbitalType::P, 2),
                ]
            }
            Self::Mc => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(7, AtomicOrbitalType::P, 3),
                ]
            }
            Self::Lv => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(7, AtomicOrbitalType::P, 4),
                ]
            }
            Self::Ts => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(7, AtomicOrbitalType::P, 5),
                ]
            }
            Self::Og => {
                vec![
                    AtomicOrbital::new(1, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(2, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(3, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(3, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(4, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(4, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(4, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(5, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(5, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(5, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(5, AtomicOrbitalType::F, 14),
                    AtomicOrbital::new(6, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(6, AtomicOrbitalType::P, 6),
                    AtomicOrbital::new(6, AtomicOrbitalType::D, 10),
                    AtomicOrbital::new(7, AtomicOrbitalType::S, 2),
                    AtomicOrbital::new(7, AtomicOrbitalType::P, 6),
                ]
            }
        }
    }
}
