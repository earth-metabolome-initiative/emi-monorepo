use std::fmt::Display;
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Atoms {
    H,
    He,
    Li,
    Be,
    B,
    C,
    N,
    O,
    F,
    Ne,
    Na,
    Mg,
    Al,
    Si,
    P,
    S,
    Cl,
    Ar,
    K,
    Ca,
    Sc,
    Ti,
    V,
    Cr,
    Mn,
    Fe,
    Co,
    Ni,
    Cu,
    Zn,
    Ga,
    Ge,
    As,
    Se,
    Br,
    Kr,
    Rb,
    Sr,
    Y,
    Zr,
    Nb,
    Mo,
    Tc,
    Ru,
    Rh,
    Pd,
    Ag,
    Cd,
    In,
    Sn,
    Sb,
    Te,
    I,
    Xe,
    Cs,
    Ba,
    La,
    Ce,
    Pr,
    Nd,
    Pm,
    Sm,
    Eu,
    Gd,
    Tb,
    Dy,
    Ho,
    Er,
    Tm,
    Yb,
    Lu,
    Hf,
    Ta,
    W,
    Re,
    Os,
    Ir,
    Pt,
    Au,
    Hg,
    Tl,
    Pb,
    Bi,
    Po,
    At,
    Rn,
    Fr,
    Ra,
    Ac,
    Th,
    Pa,
    U,
    Np,
    Pu,
    Am,
    Cm,
    Bk,
    Cf,
    Es,
    Fm,
    Md,
    No,
    Lr,
    Rf,
    Db,
    Sg,
    Bh,
    Hs,
    Mt,
    Ds,
    Rg,
    Cn,
    Nh,
    Fl,
    Mc,
    Lv,
    Ts,
    Og,
}

impl Display for Atoms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atoms::H => write!(f, "H"),
            Atoms::He => write!(f, "He"),
            Atoms::Li => write!(f, "Li"),
            Atoms::Be => write!(f, "Be"),
            Atoms::B => write!(f, "B"),
            Atoms::C => write!(f, "C"),
            Atoms::N => write!(f, "N"),
            Atoms::O => write!(f, "O"),
            Atoms::F => write!(f, "F"),
            Atoms::Ne => write!(f, "Ne"),
            Atoms::Na => write!(f, "Na"),
            Atoms::Mg => write!(f, "Mg"),
            Atoms::Al => write!(f, "Al"),
            Atoms::Si => write!(f, "Si"),
            Atoms::P => write!(f, "P"),
            Atoms::S => write!(f, "S"),
            Atoms::Cl => write!(f, "Cl"),
            Atoms::Ar => write!(f, "Ar"),
            Atoms::K => write!(f, "K"),
            Atoms::Ca => write!(f, "Ca"),
            Atoms::Sc => write!(f, "Sc"),
            Atoms::Ti => write!(f, "Ti"),
            Atoms::V => write!(f, "V"),
            Atoms::Cr => write!(f, "Cr"),
            Atoms::Mn => write!(f, "Mn"),
            Atoms::Fe => write!(f, "Fe"),
            Atoms::Co => write!(f, "Co"),
            Atoms::Ni => write!(f, "Ni"),
            Atoms::Cu => write!(f, "Cu"),
            Atoms::Zn => write!(f, "Zn"),
            Atoms::Ga => write!(f, "Ga"),
            Atoms::Ge => write!(f, "Ge"),
            Atoms::As => write!(f, "As"),
            Atoms::Se => write!(f, "Se"),
            Atoms::Br => write!(f, "Br"),
            Atoms::Kr => write!(f, "Kr"),
            Atoms::Rb => write!(f, "Rb"),
            Atoms::Sr => write!(f, "Sr"),
            Atoms::Y => write!(f, "Y"),
            Atoms::Zr => write!(f, "Zr"),
            Atoms::Nb => write!(f, "Nb"),
            Atoms::Mo => write!(f, "Mo"),
            Atoms::Tc => write!(f, "Tc"),
            Atoms::Ru => write!(f, "Ru"),
            Atoms::Rh => write!(f, "Rh"),
            Atoms::Pd => write!(f, "Pd"),
            Atoms::Ag => write!(f, "Ag"),
            Atoms::Cd => write!(f, "Cd"),
            Atoms::In => write!(f, "In"),
            Atoms::Sn => write!(f, "Sn"),
            Atoms::Sb => write!(f, "Sb"),
            Atoms::Te => write!(f, "Te"),
            Atoms::I => write!(f, "I"),
            Atoms::Xe => write!(f, "Xe"),
            Atoms::Cs => write!(f, "Cs"),
            Atoms::Ba => write!(f, "Ba"),
            Atoms::La => write!(f, "La"),
            Atoms::Ce => write!(f, "Ce"),
            Atoms::Pr => write!(f, "Pr"),
            Atoms::Nd => write!(f, "Nd"),
            Atoms::Pm => write!(f, "Pm"),
            Atoms::Sm => write!(f, "Sm"),
            Atoms::Eu => write!(f, "Eu"),
            Atoms::Gd => write!(f, "Gd"),
            Atoms::Tb => write!(f, "Tb"),
            Atoms::Dy => write!(f, "Dy"),
            Atoms::Ho => write!(f, "Ho"),
            Atoms::Er => write!(f, "Er"),
            Atoms::Tm => write!(f, "Tm"),
            Atoms::Yb => write!(f, "Yb"),
            Atoms::Lu => write!(f, "Lu"),
            Atoms::Hf => write!(f, "Hf"),
            Atoms::Ta => write!(f, "Ta"),
            Atoms::W => write!(f, "W"),
            Atoms::Re => write!(f, "Re"),
            Atoms::Os => write!(f, "Os"),
            Atoms::Ir => write!(f, "Ir"),
            Atoms::Pt => write!(f, "Pt"),
            Atoms::Au => write!(f, "Au"),
            Atoms::Hg => write!(f, "Hg"),
            Atoms::Tl => write!(f, "Tl"),
            Atoms::Pb => write!(f, "Pb"),
            Atoms::Bi => write!(f, "Bi"),
            Atoms::Po => write!(f, "Po"),
            Atoms::At => write!(f, "At"),
            Atoms::Rn => write!(f, "Rn"),
            Atoms::Fr => write!(f, "Fr"),
            Atoms::Ra => write!(f, "Ra"),
            Atoms::Ac => write!(f, "Ac"),
            Atoms::Th => write!(f, "Th"),
            Atoms::Pa => write!(f, "Pa"),
            Atoms::U => write!(f, "U"),
            Atoms::Np => write!(f, "Np"),
            Atoms::Pu => write!(f, "Pu"),
            Atoms::Am => write!(f, "Am"),
            Atoms::Cm => write!(f, "Cm"),
            Atoms::Bk => write!(f, "Bk"),
            Atoms::Cf => write!(f, "Cf"),
            Atoms::Es => write!(f, "Es"),
            Atoms::Fm => write!(f, "Fm"),
            Atoms::Md => write!(f, "Md"),
            Atoms::No => write!(f, "No"),
            Atoms::Lr => write!(f, "Lr"),
            Atoms::Rf => write!(f, "Rf"),
            Atoms::Db => write!(f, "Db"),
            Atoms::Sg => write!(f, "Sg"),
            Atoms::Bh => write!(f, "Bh"),
            Atoms::Hs => write!(f, "Hs"),
            Atoms::Mt => write!(f, "Mt"),
            Atoms::Ds => write!(f, "Ds"),
            Atoms::Rg => write!(f, "Rg"),
            Atoms::Cn => write!(f, "Cn"),
            Atoms::Nh => write!(f, "Nh"),
            Atoms::Fl => write!(f, "Fl"),
            Atoms::Mc => write!(f, "Mc"),
            Atoms::Lv => write!(f, "Lv"),
            Atoms::Ts => write!(f, "Ts"),
            Atoms::Og => write!(f, "Og"),
        }
    }
}

impl<'a> TryFrom<&'a str> for Atoms {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "H" => Ok(Atoms::H),
            "He" => Ok(Atoms::He),
            "Li" => Ok(Atoms::Li),
            "Be" => Ok(Atoms::Be),
            "B" => Ok(Atoms::B),
            "C" => Ok(Atoms::C),
            "N" => Ok(Atoms::N),
            "O" => Ok(Atoms::O),
            "F" => Ok(Atoms::F),
            "Ne" => Ok(Atoms::Ne),
            "Na" => Ok(Atoms::Na),
            "Mg" => Ok(Atoms::Mg),
            "Al" => Ok(Atoms::Al),
            "Si" => Ok(Atoms::Si),
            "P" => Ok(Atoms::P),
            "S" => Ok(Atoms::S),
            "Cl" => Ok(Atoms::Cl),
            "Ar" => Ok(Atoms::Ar),
            "K" => Ok(Atoms::K),
            "Ca" => Ok(Atoms::Ca),
            "Sc" => Ok(Atoms::Sc),
            "Ti" => Ok(Atoms::Ti),
            "V" => Ok(Atoms::V),
            "Cr" => Ok(Atoms::Cr),
            "Mn" => Ok(Atoms::Mn),
            "Fe" => Ok(Atoms::Fe),
            "Co" => Ok(Atoms::Co),
            "Ni" => Ok(Atoms::Ni),
            "Cu" => Ok(Atoms::Cu),
            "Zn" => Ok(Atoms::Zn),
            "Ga" => Ok(Atoms::Ga),
            "Ge" => Ok(Atoms::Ge),
            "As" => Ok(Atoms::As),
            "Se" => Ok(Atoms::Se),
            "Br" => Ok(Atoms::Br),
            "Kr" => Ok(Atoms::Kr),
            "Rb" => Ok(Atoms::Rb),
            "Sr" => Ok(Atoms::Sr),
            "Y" => Ok(Atoms::Y),
            "Zr" => Ok(Atoms::Zr),
            "Nb" => Ok(Atoms::Nb),
            "Mo" => Ok(Atoms::Mo),
            "Tc" => Ok(Atoms::Tc),
            "Ru" => Ok(Atoms::Ru),
            "Rh" => Ok(Atoms::Rh),
            "Pd" => Ok(Atoms::Pd),
            "Ag" => Ok(Atoms::Ag),
            "Cd" => Ok(Atoms::Cd),
            "In" => Ok(Atoms::In),
            "Sn" => Ok(Atoms::Sn),
            "Sb" => Ok(Atoms::Sb),
            "Te" => Ok(Atoms::Te),
            "I" => Ok(Atoms::I),
            "Xe" => Ok(Atoms::Xe),
            "Cs" => Ok(Atoms::Cs),
            "Ba" => Ok(Atoms::Ba),
            "La" => Ok(Atoms::La),
            "Ce" => Ok(Atoms::Ce),
            "Pr" => Ok(Atoms::Pr),
            "Nd" => Ok(Atoms::Nd),
            "Pm" => Ok(Atoms::Pm),
            "Sm" => Ok(Atoms::Sm),
            "Eu" => Ok(Atoms::Eu),
            "Gd" => Ok(Atoms::Gd),
            "Tb" => Ok(Atoms::Tb),
            "Dy" => Ok(Atoms::Dy),
            "Ho" => Ok(Atoms::Ho),
            "Er" => Ok(Atoms::Er),
            "Tm" => Ok(Atoms::Tm),
            "Yb" => Ok(Atoms::Yb),
            "Lu" => Ok(Atoms::Lu),
            "Hf" => Ok(Atoms::Hf),
            "Ta" => Ok(Atoms::Ta),
            "W" => Ok(Atoms::W),
            "Re" => Ok(Atoms::Re),
            "Os" => Ok(Atoms::Os),
            "Ir" => Ok(Atoms::Ir),
            "Pt" => Ok(Atoms::Pt),
            "Au" => Ok(Atoms::Au),
            "Hg" => Ok(Atoms::Hg),
            "Tl" => Ok(Atoms::Tl),
            "Pb" => Ok(Atoms::Pb),
            "Bi" => Ok(Atoms::Bi),
            "Po" => Ok(Atoms::Po),
            "At" => Ok(Atoms::At),
            "Rn" => Ok(Atoms::Rn),
            "Fr" => Ok(Atoms::Fr),
            "Ra" => Ok(Atoms::Ra),
            "Ac" => Ok(Atoms::Ac),
            "Th" => Ok(Atoms::Th),
            "Pa" => Ok(Atoms::Pa),
            "U" => Ok(Atoms::U),
            "Np" => Ok(Atoms::Np),
            "Pu" => Ok(Atoms::Pu),
            "Am" => Ok(Atoms::Am),
            "Cm" => Ok(Atoms::Cm),
            "Bk" => Ok(Atoms::Bk),
            "Cf" => Ok(Atoms::Cf),
            "Es" => Ok(Atoms::Es),
            "Fm" => Ok(Atoms::Fm),
            "Md" => Ok(Atoms::Md),
            "No" => Ok(Atoms::No),
            "Lr" => Ok(Atoms::Lr),
            "Rf" => Ok(Atoms::Rf),
            "Db" => Ok(Atoms::Db),
            "Sg" => Ok(Atoms::Sg),
            "Bh" => Ok(Atoms::Bh),
            "Hs" => Ok(Atoms::Hs),
            "Mt" => Ok(Atoms::Mt),
            "Ds" => Ok(Atoms::Ds),
            "Rg" => Ok(Atoms::Rg),
            "Cn" => Ok(Atoms::Cn),
            "Nh" => Ok(Atoms::Nh),
            "Fl" => Ok(Atoms::Fl),
            "Mc" => Ok(Atoms::Mc),
            "Lv" => Ok(Atoms::Lv),
            "Ts" => Ok(Atoms::Ts),
            "Og" => Ok(Atoms::Og),
            _ => Err(format!("Unknown atom: {}", s)),
        }
    }
}

impl TryFrom<String> for Atoms {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Atoms::try_from(s.as_str())
    }
}

#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AtomVector(Vec<Atoms>);

impl Display for AtomVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut atoms = self.0.iter();
        if let Some(atom) = atoms.next() {
            write!(f, "{}", atom)?;
            for atom in atoms {
                write!(f, ",{}", atom)?;
            }
        }
        Ok(())
    }
}

impl AtomVector {
    pub fn new(atoms: Vec<Atoms>) -> Self {
        AtomVector(atoms)
    }
}

impl<'a> TryFrom<&'a str> for AtomVector {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let atoms = s
            .split(',')
            .map(|atom| {
                Atoms::try_from(atom).map_err(|e| {
                    format!(
                        "Cannot parse atom: {} ({}). Maybe forgot to put a comma between atoms ?",
                        atom, e
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(AtomVector(atoms))
    }
}

impl TryFrom<String> for AtomVector {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        AtomVector::try_from(s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atoms() {
        assert_eq!(Atoms::try_from("H").unwrap(), Atoms::H);
        assert_eq!(Atoms::try_from("He").unwrap(), Atoms::He);
        assert_eq!(Atoms::try_from("Li").unwrap(), Atoms::Li);
        assert_eq!(Atoms::try_from("Be").unwrap(), Atoms::Be);
        assert_eq!(Atoms::try_from("B").unwrap(), Atoms::B);
        assert_eq!(Atoms::try_from("C").unwrap(), Atoms::C);
        assert_eq!(Atoms::try_from("N").unwrap(), Atoms::N);
        assert_eq!(Atoms::try_from("O").unwrap(), Atoms::O);
        assert_eq!(Atoms::try_from("F").unwrap(), Atoms::F);
        assert_eq!(Atoms::try_from("Ne").unwrap(), Atoms::Ne);
        assert_eq!(Atoms::try_from("Na").unwrap(), Atoms::Na);
        assert_eq!(Atoms::try_from("Mg").unwrap(), Atoms::Mg);
        assert_eq!(Atoms::try_from("Al").unwrap(), Atoms::Al);
        assert_eq!(Atoms::try_from("Si").unwrap(), Atoms::Si);
        assert_eq!(Atoms::try_from("P").unwrap(), Atoms::P);
        assert_eq!(Atoms::try_from("S").unwrap(), Atoms::S);
        assert_eq!(Atoms::try_from("Cl").unwrap(), Atoms::Cl);
        assert_eq!(Atoms::try_from("Ar").unwrap(), Atoms::Ar);
        assert_eq!(Atoms::try_from("K").unwrap(), Atoms::K);
        assert_eq!(Atoms::try_from("Ca").unwrap(), Atoms::Ca);
        assert_eq!(Atoms::try_from("Sc").unwrap(), Atoms::Sc);
        assert_eq!(Atoms::try_from("Ti").unwrap(), Atoms::Ti);
        assert_eq!(Atoms::try_from("V").unwrap(), Atoms::V);
        assert_eq!(Atoms::try_from("Cr").unwrap(), Atoms::Cr);
        assert_eq!(Atoms::try_from("Mn").unwrap(), Atoms::Mn);
        assert_eq!(Atoms::try_from("Fe").unwrap(), Atoms::Fe);
    }
    #[test]
    #[should_panic]
    fn test_from_string_fail() {
        assert_eq!(
            AtomVector::try_from("H He Li Be B C N O F Ne").unwrap(),
            AtomVector::new(vec![
                Atoms::H,
                Atoms::He,
                Atoms::Li,
                Atoms::Be,
                Atoms::B,
                Atoms::C,
                Atoms::N,
                Atoms::O,
                Atoms::F,
                Atoms::Ne
            ])
        );
    }
    #[test]
    fn test_from_string() {
        assert_eq!(
            AtomVector::try_from("H,He,Li,Be,B,C,N,O,F,Ne").unwrap(),
            AtomVector::new(vec![
                Atoms::H,
                Atoms::He,
                Atoms::Li,
                Atoms::Be,
                Atoms::B,
                Atoms::C,
                Atoms::N,
                Atoms::O,
                Atoms::F,
                Atoms::Ne
            ])
        );
        assert_eq!(
            AtomVector::try_from("N").unwrap(),
            AtomVector::new(vec![Atoms::N])
        );
        assert_eq!(
            AtomVector::try_from("N,O").unwrap(),
            AtomVector::new(vec![Atoms::N, Atoms::O])
        );
    }
}
