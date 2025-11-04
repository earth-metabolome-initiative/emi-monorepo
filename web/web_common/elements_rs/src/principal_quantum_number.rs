//! Principal quantum number (electron shell number) for elements.

use crate::isotopes::ElementVariant;

/// Number of electron shells in the atom.
pub trait PrincipalQuantumNumber {
    /// Returns the principal quantum number (shell number).
    fn principal_quantum_number(&self) -> u8;
}

impl PrincipalQuantumNumber for crate::Element {
    #[allow(clippy::too_many_lines)]
    fn principal_quantum_number(&self) -> u8 {
        match self {
            Self::H | Self::He => 1,
            Self::Li | Self::Be | Self::B | Self::C | Self::N | Self::O | Self::F | Self::Ne => 2,
            Self::Na | Self::Mg | Self::Al | Self::Si | Self::P | Self::S | Self::Cl | Self::Ar => {
                3
            }
            Self::K
            | Self::Ca
            | Self::Sc
            | Self::Ti
            | Self::V
            | Self::Cr
            | Self::Mn
            | Self::Fe
            | Self::Co
            | Self::Ni
            | Self::Cu
            | Self::Zn
            | Self::Ga
            | Self::Ge
            | Self::As
            | Self::Se
            | Self::Br
            | Self::Kr => 4,
            Self::Rb
            | Self::Sr
            | Self::Y
            | Self::Zr
            | Self::Nb
            | Self::Mo
            | Self::Tc
            | Self::Ru
            | Self::Rh
            | Self::Pd
            | Self::Ag
            | Self::Cd
            | Self::In
            | Self::Sn
            | Self::Sb
            | Self::Te
            | Self::I
            | Self::Xe => 5,
            Self::Cs
            | Self::Ba
            | Self::La
            | Self::Ce
            | Self::Pr
            | Self::Nd
            | Self::Pm
            | Self::Sm
            | Self::Eu
            | Self::Gd
            | Self::Tb
            | Self::Dy
            | Self::Ho
            | Self::Er
            | Self::Tm
            | Self::Yb
            | Self::Lu
            | Self::Hf
            | Self::Ta
            | Self::W
            | Self::Re
            | Self::Os
            | Self::Ir
            | Self::Pt
            | Self::Au
            | Self::Hg
            | Self::Tl
            | Self::Pb
            | Self::Bi
            | Self::Po
            | Self::At
            | Self::Rn => 6,
            Self::Fr
            | Self::Ra
            | Self::Ac
            | Self::Th
            | Self::Pa
            | Self::U
            | Self::Np
            | Self::Pu
            | Self::Am
            | Self::Cm
            | Self::Bk
            | Self::Cf
            | Self::Es
            | Self::Fm
            | Self::Md
            | Self::No
            | Self::Lr
            | Self::Rf
            | Self::Db
            | Self::Sg
            | Self::Bh
            | Self::Hs
            | Self::Mt
            | Self::Ds
            | Self::Rg
            | Self::Cn
            | Self::Nh
            | Self::Fl
            | Self::Mc
            | Self::Lv
            | Self::Ts
            | Self::Og => 7,
        }
    }
}

impl PrincipalQuantumNumber for crate::Isotope {
    fn principal_quantum_number(&self) -> u8 {
        self.element().principal_quantum_number()
    }
}
