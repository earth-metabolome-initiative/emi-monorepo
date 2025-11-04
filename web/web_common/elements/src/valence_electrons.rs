//! Valence electrons (outermost shell electrons) for chemical elements.

use crate::isotopes::ElementVariant;

/// Number of electrons in the outermost shell that participate in bonding.
pub trait ValenceElectrons: Sized {
    /// Returns the number of valence electrons.
    fn valence_electrons(&self) -> u8;
}

impl ValenceElectrons for crate::Element {
    fn valence_electrons(&self) -> u8 {
        match self {
            Self::H | Self::Li | Self::Na | Self::K | Self::Rb | Self::Cs | Self::Fr | Self::Ra => {
                1
            }
            Self::He | Self::Be | Self::Mg | Self::Ca | Self::Sr | Self::Ba => 2,
            Self::B
            | Self::Al
            | Self::Sc
            | Self::Ga
            | Self::Y
            | Self::In
            | Self::La
            | Self::Lu
            | Self::Tl
            | Self::Ac
            | Self::Lr
            | Self::Nh => 3,
            Self::C
            | Self::Si
            | Self::Ti
            | Self::Ge
            | Self::Zr
            | Self::Sn
            | Self::Ce
            | Self::Hf
            | Self::Pb
            | Self::Th
            | Self::Rf
            | Self::Fl => 4,
            Self::N
            | Self::P
            | Self::V
            | Self::As
            | Self::Nb
            | Self::Sb
            | Self::Pr
            | Self::Ta
            | Self::Bi
            | Self::Pa
            | Self::Db
            | Self::Mc => 5,
            Self::O
            | Self::S
            | Self::Cr
            | Self::Se
            | Self::Mo
            | Self::Te
            | Self::Nd
            | Self::W
            | Self::Po
            | Self::U
            | Self::Sg
            | Self::Lv => 6,
            Self::F
            | Self::Cl
            | Self::Mn
            | Self::Br
            | Self::Tc
            | Self::I
            | Self::Pm
            | Self::Re
            | Self::At
            | Self::Np
            | Self::Bh
            | Self::Ts => 7,
            Self::Ne
            | Self::Ar
            | Self::Fe
            | Self::Kr
            | Self::Ru
            | Self::Xe
            | Self::Sm
            | Self::Os
            | Self::Rn
            | Self::Pu
            | Self::Hs
            | Self::Og => 8,
            Self::Co | Self::Rh | Self::Eu | Self::Ir | Self::Am | Self::Mt => 9,
            Self::Ni | Self::Pd | Self::Gd | Self::Pt | Self::Cm | Self::Ds => 10,
            Self::Cu | Self::Ag | Self::Tb | Self::Au | Self::Bk | Self::Rg => 11,
            Self::Zn | Self::Cd | Self::Dy | Self::Hg | Self::Cf | Self::Cn => 12,
            Self::Ho | Self::Es => 13,
            Self::Er | Self::Fm => 14,
            Self::Tm | Self::Md => 15,
            Self::Yb | Self::No => 16,
        }
    }
}

impl ValenceElectrons for crate::Isotope {
    fn valence_electrons(&self) -> u8 {
        self.element().valence_electrons()
    }
}
