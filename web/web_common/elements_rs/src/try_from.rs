//!  Submodule provding implementations of the `TryFrom` trait for the
//! [`Element`] enum.

impl TryFrom<char> for crate::Element {
    type Error = crate::errors::Error;

    /// Parses single-character element symbols.
    ///
    /// # Implementation details
    ///
    /// It supports both uppercase and lowercase letters since,
    /// while in chemical formulas element symbols are capitalized,
    /// in other contexts such as SMILES strings they may appear in lowercase
    /// to represent aromatic atoms.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'H' => Self::H,
            'B' | 'b' => Self::B,
            'C' | 'c' => Self::C,
            'N' | 'n' => Self::N,
            'O' | 'o' => Self::O,
            'F' => Self::F,
            'P' | 'p' => Self::P,
            'S' | 's' => Self::S,
            'K' => Self::K,
            'V' => Self::V,
            'Y' => Self::Y,
            'I' => Self::I,
            'W' => Self::W,
            'U' => Self::U,
            _ => {
                return Err(crate::errors::Error::Element([value, ' ']));
            }
        })
    }
}

impl TryFrom<[char; 2]> for crate::Element {
    type Error = crate::errors::Error;

    #[allow(clippy::too_many_lines)]
    fn try_from(value: [char; 2]) -> Result<Self, Self::Error> {
        Ok(match value {
            ['H', 'e'] => Self::He,
            ['L', 'i'] => Self::Li,
            ['B', 'e'] => Self::Be,
            ['N', 'e'] => Self::Ne,
            ['N', 'a'] => Self::Na,
            ['M', 'g'] => Self::Mg,
            ['A', 'l'] => Self::Al,
            ['S', 'i'] => Self::Si,
            ['C', 'l'] => Self::Cl,
            ['A', 'r'] => Self::Ar,
            ['C', 'a'] => Self::Ca,
            ['S', 'c'] => Self::Sc,
            ['T', 'i'] => Self::Ti,
            ['C', 'r'] => Self::Cr,
            ['M', 'n'] => Self::Mn,
            ['F', 'e'] => Self::Fe,
            ['C', 'o'] => Self::Co,
            ['N', 'i'] => Self::Ni,
            ['C', 'u'] => Self::Cu,
            ['Z', 'n'] => Self::Zn,
            ['G', 'a'] => Self::Ga,
            ['G', 'e'] => Self::Ge,
            ['A' | 'a', 's'] => Self::As,
            ['S' | 's', 'e'] => Self::Se,
            ['B', 'r'] => Self::Br,
            ['K', 'r'] => Self::Kr,
            ['R', 'b'] => Self::Rb,
            ['S', 'r'] => Self::Sr,
            ['Z', 'r'] => Self::Zr,
            ['N', 'b'] => Self::Nb,
            ['M', 'o'] => Self::Mo,
            ['T', 'c'] => Self::Tc,
            ['R', 'u'] => Self::Ru,
            ['R', 'h'] => Self::Rh,
            ['P', 'd'] => Self::Pd,
            ['A', 'g'] => Self::Ag,
            ['C', 'd'] => Self::Cd,
            ['I', 'n'] => Self::In,
            ['S', 'n'] => Self::Sn,
            ['S', 'b'] => Self::Sb,
            ['T', 'e'] => Self::Te,
            ['X', 'e'] => Self::Xe,
            ['C', 's'] => Self::Cs,
            ['B', 'a'] => Self::Ba,
            ['L', 'a'] => Self::La,
            ['C', 'e'] => Self::Ce,
            ['P', 'r'] => Self::Pr,
            ['N', 'd'] => Self::Nd,
            ['P', 'm'] => Self::Pm,
            ['S', 'm'] => Self::Sm,
            ['E', 'u'] => Self::Eu,
            ['G', 'd'] => Self::Gd,
            ['T', 'b'] => Self::Tb,
            ['D', 'y'] => Self::Dy,
            ['H', 'o'] => Self::Ho,
            ['E', 'r'] => Self::Er,
            ['T', 'm'] => Self::Tm,
            ['Y', 'b'] => Self::Yb,
            ['L', 'u'] => Self::Lu,
            ['H', 'f'] => Self::Hf,
            ['T', 'a'] => Self::Ta,
            ['R', 'e'] => Self::Re,
            ['O', 's'] => Self::Os,
            ['I', 'r'] => Self::Ir,
            ['P', 't'] => Self::Pt,
            ['A', 'u'] => Self::Au,
            ['H', 'g'] => Self::Hg,
            ['T', 'l'] => Self::Tl,
            ['P', 'b'] => Self::Pb,
            ['B', 'i'] => Self::Bi,
            ['P', 'o'] => Self::Po,
            ['A', 't'] => Self::At,
            ['R', 'n'] => Self::Rn,
            ['F', 'r'] => Self::Fr,
            ['R', 'a'] => Self::Ra,
            ['A', 'c'] => Self::Ac,
            ['T', 'h'] => Self::Th,
            ['P', 'a'] => Self::Pa,
            ['N', 'p'] => Self::Np,
            ['P', 'u'] => Self::Pu,
            ['A', 'm'] => Self::Am,
            ['C', 'm'] => Self::Cm,
            ['B', 'k'] => Self::Bk,
            ['C', 'f'] => Self::Cf,
            ['E', 's'] => Self::Es,
            ['F', 'm'] => Self::Fm,
            ['M', 'd'] => Self::Md,
            ['N', 'o'] => Self::No,
            ['L', 'r'] => Self::Lr,
            ['R', 'f'] => Self::Rf,
            ['D', 'b'] => Self::Db,
            ['S', 'g'] => Self::Sg,
            ['B', 'h'] => Self::Bh,
            ['H', 's'] => Self::Hs,
            ['M', 't'] => Self::Mt,
            ['D', 's'] => Self::Ds,
            ['R', 'g'] => Self::Rg,
            ['C', 'n'] => Self::Cn,
            ['N', 'h'] => Self::Nh,
            ['F', 'l'] => Self::Fl,
            ['M', 'c'] => Self::Mc,
            ['L', 'v'] => Self::Lv,
            ['T', 's'] => Self::Ts,
            ['O', 'g'] => Self::Og,
            _ => {
                return Err(crate::errors::Error::Element(value));
            }
        })
    }
}

impl TryFrom<&str> for crate::Element {
    type Error = crate::errors::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        <Self as std::str::FromStr>::from_str(value)
    }
}

impl TryFrom<String> for crate::Element {
    type Error = crate::errors::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
