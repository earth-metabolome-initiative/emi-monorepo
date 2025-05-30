//! Submodule implementing the `isotopes` method for the `Element` enumeration.

use strum::IntoEnumIterator;

impl crate::Element {
    #[allow(clippy::too_many_lines)]
    /// Returns the isotopes of the element.
    pub fn isotopes(&self) -> Vec<super::Isotope> {
        match self {
            Self::H => super::HydrogenIsotope::iter().map(super::Isotope::H).collect(),
            Self::He => super::HeliumIsotope::iter().map(super::Isotope::He).collect(),
            Self::Li => super::LithiumIsotope::iter().map(super::Isotope::Li).collect(),
            Self::Be => super::BerylliumIsotope::iter().map(super::Isotope::Be).collect(),
            Self::B => super::BoronIsotope::iter().map(super::Isotope::B).collect(),
            Self::C => super::CarbonIsotope::iter().map(super::Isotope::C).collect(),
            Self::N => super::NitrogenIsotope::iter().map(super::Isotope::N).collect(),
            Self::O => super::OxygenIsotope::iter().map(super::Isotope::O).collect(),
            Self::F => super::FluorineIsotope::iter().map(super::Isotope::F).collect(),
            Self::Ne => super::NeonIsotope::iter().map(super::Isotope::Ne).collect(),
            Self::Na => super::SodiumIsotope::iter().map(super::Isotope::Na).collect(),
            Self::Mg => super::MagnesiumIsotope::iter().map(super::Isotope::Mg).collect(),
            Self::Al => super::AluminiumIsotope::iter().map(super::Isotope::Al).collect(),
            Self::Si => super::SiliconIsotope::iter().map(super::Isotope::Si).collect(),
            Self::P => super::PhosphorusIsotope::iter().map(super::Isotope::P).collect(),
            Self::S => super::SulfurIsotope::iter().map(super::Isotope::S).collect(),
            Self::Cl => super::ChlorineIsotope::iter().map(super::Isotope::Cl).collect(),
            Self::Ar => super::ArgonIsotope::iter().map(super::Isotope::Ar).collect(),
            Self::K => super::PotassiumIsotope::iter().map(super::Isotope::K).collect(),
            Self::Ca => super::CalciumIsotope::iter().map(super::Isotope::Ca).collect(),
            Self::Sc => super::ScandiumIsotope::iter().map(super::Isotope::Sc).collect(),
            Self::Ti => super::TitaniumIsotope::iter().map(super::Isotope::Ti).collect(),
            Self::V => super::VanadiumIsotope::iter().map(super::Isotope::V).collect(),
            Self::Cr => super::ChromiumIsotope::iter().map(super::Isotope::Cr).collect(),
            Self::Mn => super::ManganeseIsotope::iter().map(super::Isotope::Mn).collect(),
            Self::Fe => super::IronIsotope::iter().map(super::Isotope::Fe).collect(),
            Self::Co => super::CobaltIsotope::iter().map(super::Isotope::Co).collect(),
            Self::Ni => super::NickelIsotope::iter().map(super::Isotope::Ni).collect(),
            Self::Cu => super::CopperIsotope::iter().map(super::Isotope::Cu).collect(),
            Self::Zn => super::ZincIsotope::iter().map(super::Isotope::Zn).collect(),
            Self::Ga => super::GalliumIsotope::iter().map(super::Isotope::Ga).collect(),
            Self::Ge => super::GermaniumIsotope::iter().map(super::Isotope::Ge).collect(),
            Self::As => super::ArsenicIsotope::iter().map(super::Isotope::As).collect(),
            Self::Se => super::SeleniumIsotope::iter().map(super::Isotope::Se).collect(),
            Self::Br => super::BromineIsotope::iter().map(super::Isotope::Br).collect(),
            Self::Kr => super::KryptonIsotope::iter().map(super::Isotope::Kr).collect(),
            Self::Rb => super::RubidiumIsotope::iter().map(super::Isotope::Rb).collect(),
            Self::Sr => super::StrontiumIsotope::iter().map(super::Isotope::Sr).collect(),
            Self::Y => super::YttriumIsotope::iter().map(super::Isotope::Y).collect(),
            Self::Zr => super::ZirconiumIsotope::iter().map(super::Isotope::Zr).collect(),
            Self::Nb => super::NiobiumIsotope::iter().map(super::Isotope::Nb).collect(),
            Self::Mo => super::MolybdenumIsotope::iter().map(super::Isotope::Mo).collect(),
            Self::Tc => super::TechnetiumIsotope::iter().map(super::Isotope::Tc).collect(),
            Self::Ru => super::RutheniumIsotope::iter().map(super::Isotope::Ru).collect(),
            Self::Rh => super::RhodiumIsotope::iter().map(super::Isotope::Rh).collect(),
            Self::Pd => super::PalladiumIsotope::iter().map(super::Isotope::Pd).collect(),
            Self::Ag => super::SilverIsotope::iter().map(super::Isotope::Ag).collect(),
            Self::Cd => super::CadmiumIsotope::iter().map(super::Isotope::Cd).collect(),
            Self::In => super::IndiumIsotope::iter().map(super::Isotope::In).collect(),
            Self::Sn => super::TinIsotope::iter().map(super::Isotope::Sn).collect(),
            Self::Sb => super::AntimonyIsotope::iter().map(super::Isotope::Sb).collect(),
            Self::Te => super::TelluriumIsotope::iter().map(super::Isotope::Te).collect(),
            Self::I => super::IodineIsotope::iter().map(super::Isotope::I).collect(),
            Self::Xe => super::XenonIsotope::iter().map(super::Isotope::Xe).collect(),
            Self::Cs => super::CaesiumIsotope::iter().map(super::Isotope::Cs).collect(),
            Self::Ba => super::BariumIsotope::iter().map(super::Isotope::Ba).collect(),
            Self::La => super::LanthanumIsotope::iter().map(super::Isotope::La).collect(),
            Self::Ce => super::CeriumIsotope::iter().map(super::Isotope::Ce).collect(),
            Self::Pr => super::PraseodymiumIsotope::iter().map(super::Isotope::Pr).collect(),
            Self::Nd => super::NeodymiumIsotope::iter().map(super::Isotope::Nd).collect(),
            Self::Pm => super::PromethiumIsotope::iter().map(super::Isotope::Pm).collect(),
            Self::Sm => super::SamariumIsotope::iter().map(super::Isotope::Sm).collect(),
            Self::Eu => super::EuropiumIsotope::iter().map(super::Isotope::Eu).collect(),
            Self::Gd => super::GadoliniumIsotope::iter().map(super::Isotope::Gd).collect(),
            Self::Tb => super::TerbiumIsotope::iter().map(super::Isotope::Tb).collect(),
            Self::Dy => super::DysprosiumIsotope::iter().map(super::Isotope::Dy).collect(),
            Self::Ho => super::HolmiumIsotope::iter().map(super::Isotope::Ho).collect(),
            Self::Er => super::ErbiumIsotope::iter().map(super::Isotope::Er).collect(),
            Self::Tm => super::ThuliumIsotope::iter().map(super::Isotope::Tm).collect(),
            Self::Yb => super::YtterbiumIsotope::iter().map(super::Isotope::Yb).collect(),
            Self::Lu => super::LutetiumIsotope::iter().map(super::Isotope::Lu).collect(),
            Self::Hf => super::HafniumIsotope::iter().map(super::Isotope::Hf).collect(),
            Self::Ta => super::TantalumIsotope::iter().map(super::Isotope::Ta).collect(),
            Self::W => super::TungstenIsotope::iter().map(super::Isotope::W).collect(),
            Self::Re => super::RheniumIsotope::iter().map(super::Isotope::Re).collect(),
            Self::Os => super::OsmiumIsotope::iter().map(super::Isotope::Os).collect(),
            Self::Ir => super::IridiumIsotope::iter().map(super::Isotope::Ir).collect(),
            Self::Pt => super::PlatinumIsotope::iter().map(super::Isotope::Pt).collect(),
            Self::Au => super::GoldIsotope::iter().map(super::Isotope::Au).collect(),
            Self::Hg => super::MercuryIsotope::iter().map(super::Isotope::Hg).collect(),
            Self::Tl => super::ThalliumIsotope::iter().map(super::Isotope::Tl).collect(),
            Self::Pb => super::LeadIsotope::iter().map(super::Isotope::Pb).collect(),
            Self::Bi => super::BismuthIsotope::iter().map(super::Isotope::Bi).collect(),
            Self::Po => super::PoloniumIsotope::iter().map(super::Isotope::Po).collect(),
            Self::At => super::AstatineIsotope::iter().map(super::Isotope::At).collect(),
            Self::Rn => super::RadonIsotope::iter().map(super::Isotope::Rn).collect(),
            Self::Fr => super::FranciumIsotope::iter().map(super::Isotope::Fr).collect(),
            Self::Ra => super::RadiumIsotope::iter().map(super::Isotope::Ra).collect(),
            Self::Ac => super::ActiniumIsotope::iter().map(super::Isotope::Ac).collect(),
            Self::Th => super::ThoriumIsotope::iter().map(super::Isotope::Th).collect(),
            Self::Pa => super::ProtactiniumIsotope::iter().map(super::Isotope::Pa).collect(),
            Self::U => super::UraniumIsotope::iter().map(super::Isotope::U).collect(),
            Self::Np => super::NeptuniumIsotope::iter().map(super::Isotope::Np).collect(),
            Self::Pu => super::PlutoniumIsotope::iter().map(super::Isotope::Pu).collect(),
            Self::Am => super::AmericiumIsotope::iter().map(super::Isotope::Am).collect(),
            Self::Cm => super::CuriumIsotope::iter().map(super::Isotope::Cm).collect(),
            Self::Bk => super::BerkeliumIsotope::iter().map(super::Isotope::Bk).collect(),
            Self::Cf => super::CaliforniumIsotope::iter().map(super::Isotope::Cf).collect(),
            Self::Es => super::EinsteiniumIsotope::iter().map(super::Isotope::Es).collect(),
            Self::Fm => super::FermiumIsotope::iter().map(super::Isotope::Fm).collect(),
            Self::Md => super::MendeleviumIsotope::iter().map(super::Isotope::Md).collect(),
            Self::No => super::NobeliumIsotope::iter().map(super::Isotope::No).collect(),
            Self::Lr => super::LawrenciumIsotope::iter().map(super::Isotope::Lr).collect(),
            Self::Rf => super::RutherfordiumIsotope::iter().map(super::Isotope::Rf).collect(),
            Self::Db => super::DubniumIsotope::iter().map(super::Isotope::Db).collect(),
            Self::Sg => super::SeaborgiumIsotope::iter().map(super::Isotope::Sg).collect(),
            Self::Bh => super::BohriumIsotope::iter().map(super::Isotope::Bh).collect(),
            Self::Hs => super::HassiumIsotope::iter().map(super::Isotope::Hs).collect(),
            Self::Mt => super::MeitneriumIsotope::iter().map(super::Isotope::Mt).collect(),
            Self::Ds => super::DarmstadtiumIsotope::iter().map(super::Isotope::Ds).collect(),
            Self::Rg => super::RoentgeniumIsotope::iter().map(super::Isotope::Rg).collect(),
            Self::Cn => super::CoperniciumIsotope::iter().map(super::Isotope::Cn).collect(),
            Self::Nh => super::NihoniumIsotope::iter().map(super::Isotope::Nh).collect(),
            Self::Fl => super::FleroviumIsotope::iter().map(super::Isotope::Fl).collect(),
            Self::Mc => super::MoscoviumIsotope::iter().map(super::Isotope::Mc).collect(),
            Self::Lv => super::LivermoriumIsotope::iter().map(super::Isotope::Lv).collect(),
            Self::Ts => super::TennessineIsotope::iter().map(super::Isotope::Ts).collect(),
            Self::Og => super::OganessonIsotope::iter().map(super::Isotope::Og).collect(),
        }
    }
}
