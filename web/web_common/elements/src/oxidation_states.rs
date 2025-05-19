//! Submodule implementing the `oxidation_states` element method.

impl super::Element {
    /// Returns whether the provided oxidation state is valid for the element.
    #[must_use]
    pub fn is_valid_oxidation_state(&self, state: i16) -> bool {
        self.oxidation_states().contains(&state)
    }

    /// Returns the allowed oxidation states of the element.
    #[must_use]
    pub fn oxidation_states(&self) -> &'static [i16] {
        match self {
            Self::B => &[-5, -1, 1, 2, 3],
            Self::C | Self::Si | Self::Ge | Self::Sn => &[-4, -3, -2, -1, 1, 2, 3, 4],
            Self::N | Self::P | Self::As | Self::Sb | Self::Bi => &[-3, -2, -1, 1, 2, 3, 4, 5],
            Self::O => &[-2, -1, 1, 2],
            Self::F => &[-1],
            Self::Al | Self::Ag => &[-2, -1, 1, 2, 3],
            Self::S | Self::Se | Self::Te => &[-2, -1, 1, 2, 3, 4, 5, 6],
            Self::Cl | Self::Tc | Self::I => &[-1, 1, 2, 3, 4, 5, 6, 7],
            Self::Ti | Self::Ni | Self::Cu => &[-2, -1, 1, 2, 3, 4],
            Self::V | Self::Co | Self::Nb | Self::Ta => &[-3, -1, 1, 2, 3, 4, 5],
            Self::Cr | Self::Mo | Self::W => &[-4, -2, -1, 1, 2, 3, 4, 5, 6],
            Self::Mn => &[-3, -2, -1, 1, 2, 3, 4, 5, 6, 7],
            Self::Fe => &[-2, -1, 1, 2, 3, 4, 5, 6, 7],
            Self::Zn | Self::Cd | Self::Hg => &[-2, 1, 2],
            Self::Ga => &[-5, -4, -3, -2, -1, 1, 2, 3],
            Self::Br => &[-1, 1, 2, 3, 4, 5, 7],
            Self::Zr | Self::Hf => &[-2, 1, 2, 3, 4],
            Self::Ru => &[-2, 1, 2, 3, 4, 5, 6, 7, 8],
            Self::Rh | Self::Re => &[-3, -1, 1, 2, 3, 4, 5, 6, 7],
            Self::Pd | Self::Pr => &[1, 2, 3, 4, 5],
            Self::In | Self::Tl => &[-5, -2, -1, 1, 2, 3],
            Self::Xe => &[2, 4, 6, 8],
            Self::Tb => &[1, 2, 3, 4],
            Self::Os => &[-4, -2, -1, 1, 2, 3, 4, 5, 6, 7, 8],
            Self::Ir => &[-3, -2, -1, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            Self::Pt => &[-3, -2, -1, 1, 2, 3, 4, 5, 6],
            Self::Au => &[-3, -2, -1, 1, 2, 3, 5],
            Self::Pb => &[-4, -2, -1, 1, 2, 3, 4],
            Self::Po => &[-2, 2, 4, 5, 6],
            Self::At => &[-1, 1, 3, 5, 7],
            Self::Rn => &[2, 6],
            Self::Fr => &[1],
            Self::Ra => &[2],
            Self::Ac | Self::Lr => &[3],
            Self::Th => &[-1, 1, 2, 3, 4],
            Self::Pa | Self::Bk | Self::Cf => &[2, 3, 4, 5],
            Self::U => &[-1, 1, 2, 3, 4, 5, 6],
            Self::Np | Self::Am => &[2, 3, 4, 5, 6, 7],
            Self::Pu => &[2, 3, 4, 5, 6, 7, 8],
            Self::Cm | Self::Sg => &[3, 4, 5, 6],
            Self::Rf => &[3, 4],
            Self::Db => &[3, 4, 5],
            Self::Bh => &[3, 4, 5, 7],
            Self::Hs => &[3, 4, 6, 8],
            Self::Mt => &[1, 3, 6],
            Self::Ds => &[2, 4, 6],
            Self::Rg => &[-1, 3, 5],
            Self::Cn => &[2, 4],
            Self::Lv => &[-2, 4],
            Self::Ts => &[-1, 5],
            Self::Og => &[-1, 1, 2, 4, 6],
            Self::Pm
            | Self::Eu
            | Self::Ho
            | Self::Er
            | Self::Lu
            | Self::Fm
            | Self::Md
            | Self::No => &[2, 3],
            Self::Ce | Self::Nd | Self::Dy | Self::Es => &[2, 3, 4],

            Self::Sc | Self::Y | Self::La | Self::Sm | Self::Gd | Self::Tm | Self::Yb => &[1, 2, 3],

            Self::Be | Self::Mg | Self::Ca | Self::Kr | Self::Sr | Self::Ba => &[1, 2],

            Self::H | Self::Li | Self::Na | Self::K | Self::Rb | Self::Cs => &[-1, 1],
            Self::He | Self::Ne | Self::Ar | Self::Nh | Self::Fl | Self::Mc => &[],
        }
    }
}
