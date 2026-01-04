//! Bonding capacity of chemical elements.

use crate::isotopes::ElementVariant;

/// Minimum and maximum number of bonds an element can form.
pub trait BondsNumber {
    /// Returns `(min_bonds, max_bonds)` for the element.
    fn number_of_bonds(&self) -> (u8, u8);

    /// Returns `true` for noble gases (elements with zero bonds).
    fn is_noble_gas(&self) -> bool {
        self.number_of_bonds() == (0, 0)
    }
}

impl BondsNumber for crate::Element {
    #[allow(clippy::match_same_arms)]
    #[allow(clippy::too_many_lines)]
    fn number_of_bonds(&self) -> (u8, u8) {
        match self {
            Self::H => (1, 1),
            Self::He => (0, 0),
            Self::Li => (1, 1),
            Self::Be => (2, 2),
            Self::B => (3, 3),
            Self::C => (4, 4),
            Self::N => (3, 3),
            Self::O => (2, 2),
            Self::F => (1, 1),
            Self::Ne => (0, 0),
            Self::Na => (1, 1),
            Self::Mg => (2, 2),
            Self::Al => (3, 3),
            Self::Si => (4, 4),
            Self::P => (3, 3),
            Self::S => (2, 2),
            Self::Cl => (1, 1),
            Self::Ar => (0, 0),
            Self::K => (1, 1),
            Self::Ca => (2, 2),
            Self::Sc => (3, 3),
            Self::Ti => (4, 4),
            Self::V => (5, 5),
            Self::Cr => (6, 6),
            Self::Mn => (7, 7),
            Self::Fe => (3, 3),
            Self::Co => (3, 3),
            Self::Ni => (2, 2),
            Self::Cu => (2, 2),
            Self::Zn => (2, 2),
            Self::Ga => (3, 3),
            Self::Ge => (4, 4),
            Self::As => (3, 3),
            Self::Se => (2, 2),
            Self::Br => (1, 1),
            Self::Kr => (0, 0),
            Self::Rb => (1, 1),
            Self::Sr => (2, 2),
            Self::Y => (3, 3),
            Self::Zr => (4, 4),
            Self::Nb => (5, 5),
            Self::Mo => (6, 6),
            Self::Tc => (7, 7),
            Self::Ru => (8, 8),
            Self::Rh => (6, 6),
            Self::Pd => (4, 4),
            Self::Ag => (1, 1),
            Self::Cd => (2, 2),
            Self::In => (3, 3),
            Self::Sn => (4, 4),
            Self::Sb => (3, 3),
            Self::Te => (2, 2),
            Self::I => (1, 1),
            Self::Xe => (0, 0),
            Self::Cs => (1, 1),
            Self::Ba => (2, 2),
            Self::La => (3, 6),
            Self::Ce => (1, 6),
            Self::Pr => (1, 6),
            Self::Nd => (1, 6),
            Self::Pm => (1, 6),
            Self::Sm => (1, 6),
            Self::Eu => (1, 6),
            Self::Gd => (1, 6),
            Self::Tb => (1, 6),
            Self::Dy => (1, 6),
            Self::Ho => (1, 6),
            Self::Er => (1, 6),
            Self::Tm => (1, 6),
            Self::Yb => (1, 6),
            Self::Lu => (1, 6),
            Self::Hf => (1, 3),
            Self::Ta => (1, 3),
            Self::W => (1, 3),
            Self::Re => (1, 3),
            Self::Os => (1, 3),
            Self::Ir => (1, 3),
            Self::Pt => (1, 3),
            Self::Au => (1, 3),
            Self::Hg => (1, 3),
            Self::Tl => (1, 3),
            Self::Pb => (1, 3),
            Self::Bi => (1, 3),
            Self::Po => (1, 3),
            Self::At => (1, 3),
            Self::Rn => (0, 0),
            Self::Fr => (1, 6),
            Self::Ra => (1, 6),
            Self::Ac => (1, 6),
            Self::Th => (1, 6),
            Self::Pa => (1, 6),
            Self::U => (1, 6),
            Self::Np => (1, 6),
            Self::Pu => (1, 6),
            Self::Am => (1, 3),
            Self::Cm => (1, 3),
            Self::Bk => (1, 3),
            Self::Cf => (1, 3),
            Self::Es => (1, 3),
            Self::Fm => (1, 3),
            Self::Md => (1, 3),
            Self::No => (1, 3),
            Self::Lr => (1, 3),
            Self::Rf => (1, 3),
            Self::Db => (1, 3),
            Self::Sg => (1, 3),
            Self::Bh => (1, 3),
            Self::Hs => (1, 3),
            Self::Mt => (1, 3),
            Self::Ds => (1, 3),
            Self::Rg => (1, 3),
            Self::Cn => (1, 3),
            Self::Nh => (1, 3),
            Self::Fl => (1, 3),
            Self::Mc => (1, 3),
            Self::Lv => (1, 3),
            Self::Ts => (1, 3),
            Self::Og => (0, 0),
        }
    }
}

impl BondsNumber for crate::Isotope {
    fn number_of_bonds(&self) -> (u8, u8) {
        self.element().number_of_bonds()
    }
}
