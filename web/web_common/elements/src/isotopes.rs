//! Submodule providing the enumeration of `Isotopes` for each element.

mod element_variant;
mod isotopic_composition;
mod mass_number;
mod relative_atomic_mass;
mod display;

pub mod actinium;
pub mod aluminium;
pub mod americium;
pub mod antimony;
pub mod argon;
pub mod arsenic;
pub mod astatine;
pub mod barium;
pub mod berkelium;
pub mod beryllium;
pub mod bismuth;
pub mod bohrium;
pub mod boron;
pub mod bromine;
pub mod cadmium;
pub mod caesium;
pub mod calcium;
pub mod californium;
pub mod carbon;
pub mod cerium;
pub mod chlorine;
pub mod chromium;
pub mod cobalt;
pub mod copernicium;
pub mod copper;
pub mod curium;
pub mod darmstadtium;
pub mod dubnium;
pub mod dysprosium;
pub mod einsteinium;
pub mod erbium;
pub mod europium;
pub mod fermium;
pub mod flerovium;
pub mod fluorine;
pub mod francium;
pub mod gadolinium;
pub mod gallium;
pub mod germanium;
pub mod gold;
pub mod hafnium;
pub mod hassium;
pub mod helium;
pub mod holmium;
pub mod hydrogen;
pub mod indium;
pub mod iodine;
pub mod iridium;
pub mod iron;
pub mod krypton;
pub mod lanthanum;
pub mod lawrencium;
pub mod lead;
pub mod lithium;
pub mod livermorium;
pub mod lutetium;
pub mod magnesium;
pub mod manganese;
pub mod meitnerium;
pub mod mendelevium;
pub mod mercury;
pub mod molybdenum;
pub mod moscovium;
pub mod neodymium;
pub mod neon;
pub mod neptunium;
pub mod nickel;
pub mod nihonium;
pub mod niobium;
pub mod nitrogen;
pub mod nobelium;
pub mod oganesson;
pub mod osmium;
pub mod oxygen;
pub mod palladium;
pub mod phosphorus;
pub mod platinum;
pub mod plutonium;
pub mod polonium;
pub mod potassium;
pub mod praseodymium;
pub mod promethium;
pub mod protactinium;
pub mod radium;
pub mod radon;
pub mod rhenium;
pub mod rhodium;
pub mod roentgenium;
pub mod rubidium;
pub mod ruthenium;
pub mod rutherfordium;
pub mod samarium;
pub mod scandium;
pub mod seaborgium;
pub mod selenium;
pub mod silicon;
pub mod silver;
pub mod sodium;
pub mod strontium;
pub mod sulfur;
pub mod tantalum;
pub mod technetium;
pub mod tellurium;
pub mod tennessine;
pub mod terbium;
pub mod thallium;
pub mod thorium;
pub mod thulium;
pub mod tin;
pub mod titanium;
pub mod tungsten;
pub mod uranium;
pub mod vanadium;
pub mod xenon;
pub mod ytterbium;
pub mod yttrium;
pub mod zinc;
pub mod zirconium;

pub use actinium::ActiniumIsotope;
pub use aluminium::AluminiumIsotope;
pub use americium::AmericiumIsotope;
pub use antimony::AntimonyIsotope;
pub use argon::ArgonIsotope;
pub use arsenic::ArsenicIsotope;
pub use astatine::AstatineIsotope;
pub use barium::BariumIsotope;
pub use berkelium::BerkeliumIsotope;
pub use beryllium::BerylliumIsotope;
pub use bismuth::BismuthIsotope;
pub use bohrium::BohriumIsotope;
pub use boron::BoronIsotope;
pub use bromine::BromineIsotope;
pub use cadmium::CadmiumIsotope;
pub use caesium::CaesiumIsotope;
pub use calcium::CalciumIsotope;
pub use californium::CaliforniumIsotope;
pub use carbon::CarbonIsotope;
pub use cerium::CeriumIsotope;
pub use chlorine::ChlorineIsotope;
pub use chromium::ChromiumIsotope;
pub use cobalt::CobaltIsotope;
pub use copernicium::CoperniciumIsotope;
pub use copper::CopperIsotope;
pub use curium::CuriumIsotope;
pub use darmstadtium::DarmstadtiumIsotope;
pub use dubnium::DubniumIsotope;
pub use dysprosium::DysprosiumIsotope;
pub use einsteinium::EinsteiniumIsotope;
pub use erbium::ErbiumIsotope;
pub use europium::EuropiumIsotope;
pub use fermium::FermiumIsotope;
pub use flerovium::FleroviumIsotope;
pub use fluorine::FluorineIsotope;
pub use francium::FranciumIsotope;
pub use gadolinium::GadoliniumIsotope;
pub use gallium::GalliumIsotope;
pub use germanium::GermaniumIsotope;
pub use gold::GoldIsotope;
pub use hafnium::HafniumIsotope;
pub use hassium::HassiumIsotope;
pub use helium::HeliumIsotope;
pub use holmium::HolmiumIsotope;
pub use hydrogen::HydrogenIsotope;
pub use indium::IndiumIsotope;
pub use iodine::IodineIsotope;
pub use iridium::IridiumIsotope;
pub use iron::IronIsotope;
pub use krypton::KryptonIsotope;
pub use lanthanum::LanthanumIsotope;
pub use lawrencium::LawrenciumIsotope;
pub use lead::LeadIsotope;
pub use lithium::LithiumIsotope;
pub use livermorium::LivermoriumIsotope;
pub use lutetium::LutetiumIsotope;
pub use magnesium::MagnesiumIsotope;
pub use manganese::ManganeseIsotope;
pub use meitnerium::MeitneriumIsotope;
pub use mendelevium::MendeleviumIsotope;
pub use mercury::MercuryIsotope;
pub use molybdenum::MolybdenumIsotope;
pub use moscovium::MoscoviumIsotope;
pub use neodymium::NeodymiumIsotope;
pub use neon::NeonIsotope;
pub use neptunium::NeptuniumIsotope;
pub use nickel::NickelIsotope;
pub use nihonium::NihoniumIsotope;
pub use niobium::NiobiumIsotope;
pub use nitrogen::NitrogenIsotope;
pub use nobelium::NobeliumIsotope;
pub use oganesson::OganessonIsotope;
pub use osmium::OsmiumIsotope;
pub use oxygen::OxygenIsotope;
pub use palladium::PalladiumIsotope;
pub use phosphorus::PhosphorusIsotope;
pub use platinum::PlatinumIsotope;
pub use plutonium::PlutoniumIsotope;
pub use polonium::PoloniumIsotope;
pub use potassium::PotassiumIsotope;
pub use praseodymium::PraseodymiumIsotope;
pub use promethium::PromethiumIsotope;
pub use protactinium::ProtactiniumIsotope;
pub use radium::RadiumIsotope;
pub use radon::RadonIsotope;
pub use rhenium::RheniumIsotope;
pub use rhodium::RhodiumIsotope;
pub use roentgenium::RoentgeniumIsotope;
pub use rubidium::RubidiumIsotope;
pub use ruthenium::RutheniumIsotope;
pub use rutherfordium::RutherfordiumIsotope;
pub use samarium::SamariumIsotope;
pub use scandium::ScandiumIsotope;
pub use seaborgium::SeaborgiumIsotope;
pub use selenium::SeleniumIsotope;
pub use silicon::SiliconIsotope;
pub use silver::SilverIsotope;
pub use sodium::SodiumIsotope;
pub use strontium::StrontiumIsotope;
pub use sulfur::SulfurIsotope;
pub use tantalum::TantalumIsotope;
pub use technetium::TechnetiumIsotope;
pub use tellurium::TelluriumIsotope;
pub use tennessine::TennessineIsotope;
pub use terbium::TerbiumIsotope;
pub use thallium::ThalliumIsotope;
pub use thorium::ThoriumIsotope;
pub use thulium::ThuliumIsotope;
pub use tin::TinIsotope;
pub use titanium::TitaniumIsotope;
pub use tungsten::TungstenIsotope;
pub use uranium::UraniumIsotope;
pub use vanadium::VanadiumIsotope;
pub use xenon::XenonIsotope;
pub use ytterbium::YtterbiumIsotope;
pub use yttrium::YttriumIsotope;
pub use zinc::ZincIsotope;
pub use zirconium::ZirconiumIsotope;

/// Trait defining the relative atomic mass of an isotope.
pub trait RelativeAtomicMass {
    /// Returns the relative atomic mass of the isotope.
    fn relative_atomic_mass(&self) -> f64;
}

/// Trait defining the mass number of an isotope.
pub trait MassNumber {
    /// Returns the mass number of the isotope.
    fn mass_number(&self) -> u16;
}

/// Trait defining the Element of an isotope.
pub trait ElementVariant {
    /// Returns the element of the isotope.
    fn element(&self) -> crate::Element;
}

/// Trait defining the isotopic composition of an isotope.
pub trait IsotopicComposition {
    /// Returns the isotopic composition of the isotope.
    fn isotopic_composition(&self) -> Option<f64>;
}

/// Trait defining the most common isotope of an element.
pub trait MostAbundantIsotope {
    /// Returns the most common isotope of the element.
    fn most_abundant_isotope() -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Enumeration of all isotopes of the elements in the periodic table.
pub enum Isotope {
    H(HydrogenIsotope),
    He(HeliumIsotope),
    Li(LithiumIsotope),
    Be(BerylliumIsotope),
    B(BoronIsotope),
    C(CarbonIsotope),
    N(NitrogenIsotope),
    O(OxygenIsotope),
    F(FluorineIsotope),
    Ne(NeonIsotope),
    Na(SodiumIsotope),
    Mg(MagnesiumIsotope),
    Al(AluminiumIsotope),
    Si(SiliconIsotope),
    P(PhosphorusIsotope),
    S(SulfurIsotope),
    Cl(ChlorineIsotope),
    Ar(ArgonIsotope),
    K(PotassiumIsotope),
    Ca(CalciumIsotope),
    Sc(ScandiumIsotope),
    Ti(TitaniumIsotope),
    V(VanadiumIsotope),
    Cr(ChromiumIsotope),
    Mn(ManganeseIsotope),
    Fe(IronIsotope),
    Co(CobaltIsotope),
    Ni(NickelIsotope),
    Cu(CopperIsotope),
    Zn(ZincIsotope),
    Ga(GalliumIsotope),
    Ge(GermaniumIsotope),
    As(ArsenicIsotope),
    Se(SeleniumIsotope),
    Br(BromineIsotope),
    Kr(KryptonIsotope),
    Rb(RubidiumIsotope),
    Sr(StrontiumIsotope),
    Y(YttriumIsotope),
    Zr(ZirconiumIsotope),
    Nb(NiobiumIsotope),
    Mo(MolybdenumIsotope),
    Tc(TechnetiumIsotope),
    Ru(RutheniumIsotope),
    Rh(RhodiumIsotope),
    Pd(PalladiumIsotope),
    Ag(SilverIsotope),
    Cd(CadmiumIsotope),
    In(IndiumIsotope),
    Sn(TinIsotope),
    Sb(AntimonyIsotope),
    Te(TelluriumIsotope),
    I(IodineIsotope),
    Xe(XenonIsotope),
    Cs(CaesiumIsotope),
    Ba(BariumIsotope),
    La(LanthanumIsotope),
    Ce(CeriumIsotope),
    Pr(PraseodymiumIsotope),
    Nd(NeodymiumIsotope),
    Pm(PromethiumIsotope),
    Sm(SamariumIsotope),
    Eu(EuropiumIsotope),
    Gd(GadoliniumIsotope),
    Tb(TerbiumIsotope),
    Dy(DysprosiumIsotope),
    Ho(HolmiumIsotope),
    Er(ErbiumIsotope),
    Tm(ThuliumIsotope),
    Yb(YtterbiumIsotope),
    Lu(LutetiumIsotope),
    Hf(HafniumIsotope),
    Ta(TantalumIsotope),
    W(TungstenIsotope),
    Re(RheniumIsotope),
    Os(OsmiumIsotope),
    Ir(IridiumIsotope),
    Pt(PlatinumIsotope),
    Au(GoldIsotope),
    Hg(MercuryIsotope),
    Tl(ThalliumIsotope),
    Pb(LeadIsotope),
    Bi(BismuthIsotope),
    Po(PoloniumIsotope),
    At(AstatineIsotope),
    Rn(RadonIsotope),
    Fr(FranciumIsotope),
    Ra(RadiumIsotope),
    Ac(ActiniumIsotope),
    Th(ThoriumIsotope),
    Pa(ProtactiniumIsotope),
    U(UraniumIsotope),
    Np(NeptuniumIsotope),
    Pu(PlutoniumIsotope),
    Am(AmericiumIsotope),
    Cm(CuriumIsotope),
    Bk(BerkeliumIsotope),
    Cf(CaliforniumIsotope),
    Es(EinsteiniumIsotope),
    Fm(FermiumIsotope),
    Md(MendeleviumIsotope),
    No(NobeliumIsotope),
    Lr(LawrenciumIsotope),
    Rf(RutherfordiumIsotope),
    Db(DubniumIsotope),
    Sg(SeaborgiumIsotope),
    Bh(BohriumIsotope),
    Hs(HassiumIsotope),
    Mt(MeitneriumIsotope),
    Ds(DarmstadtiumIsotope),
    Rg(RoentgeniumIsotope),
    Cn(CoperniciumIsotope),
    Nh(NihoniumIsotope),
    Fl(FleroviumIsotope),
    Mc(MoscoviumIsotope),
    Lv(LivermoriumIsotope),
    Ts(TennessineIsotope),
    Og(OganessonIsotope),
}

impl crate::Element {
    /// Returns the most common isotope of the element.
    pub fn most_abundant_isotope(&self) -> Isotope {
        match self {
            Self::H => Isotope::H(HydrogenIsotope::most_abundant_isotope()),
            Self::He => Isotope::He(HeliumIsotope::most_abundant_isotope()),
            Self::Li => Isotope::Li(LithiumIsotope::most_abundant_isotope()),
            Self::Be => Isotope::Be(BerylliumIsotope::most_abundant_isotope()),
            Self::B => Isotope::B(BoronIsotope::most_abundant_isotope()),
            Self::C => Isotope::C(CarbonIsotope::most_abundant_isotope()),
            Self::N => Isotope::N(NitrogenIsotope::most_abundant_isotope()),
            Self::O => Isotope::O(OxygenIsotope::most_abundant_isotope()),
            Self::F => Isotope::F(FluorineIsotope::most_abundant_isotope()),
            Self::Ne => Isotope::Ne(NeonIsotope::most_abundant_isotope()),
            Self::Na => Isotope::Na(SodiumIsotope::most_abundant_isotope()),
            Self::Mg => Isotope::Mg(MagnesiumIsotope::most_abundant_isotope()),
            Self::Al => Isotope::Al(AluminiumIsotope::most_abundant_isotope()),
            Self::Si => Isotope::Si(SiliconIsotope::most_abundant_isotope()),
            Self::P => Isotope::P(PhosphorusIsotope::most_abundant_isotope()),
            Self::S => Isotope::S(SulfurIsotope::most_abundant_isotope()),
            Self::Cl => Isotope::Cl(ChlorineIsotope::most_abundant_isotope()),
            Self::Ar => Isotope::Ar(ArgonIsotope::most_abundant_isotope()),
            Self::K => Isotope::K(PotassiumIsotope::most_abundant_isotope()),
            Self::Ca => Isotope::Ca(CalciumIsotope::most_abundant_isotope()),
            Self::Sc => Isotope::Sc(ScandiumIsotope::most_abundant_isotope()),
            Self::Ti => Isotope::Ti(TitaniumIsotope::most_abundant_isotope()),
            Self::V => Isotope::V(VanadiumIsotope::most_abundant_isotope()),
            Self::Cr => Isotope::Cr(ChromiumIsotope::most_abundant_isotope()),
            Self::Mn => Isotope::Mn(ManganeseIsotope::most_abundant_isotope()),
            Self::Fe => Isotope::Fe(IronIsotope::most_abundant_isotope()),
            Self::Co => Isotope::Co(CobaltIsotope::most_abundant_isotope()),
            Self::Ni => Isotope::Ni(NickelIsotope::most_abundant_isotope()),
            Self::Cu => Isotope::Cu(CopperIsotope::most_abundant_isotope()),
            Self::Zn => Isotope::Zn(ZincIsotope::most_abundant_isotope()),
            Self::Ga => Isotope::Ga(GalliumIsotope::most_abundant_isotope()),
            Self::Ge => Isotope::Ge(GermaniumIsotope::most_abundant_isotope()),
            Self::As => Isotope::As(ArsenicIsotope::most_abundant_isotope()),
            Self::Se => Isotope::Se(SeleniumIsotope::most_abundant_isotope()),
            Self::Br => Isotope::Br(BromineIsotope::most_abundant_isotope()),
            Self::Kr => Isotope::Kr(KryptonIsotope::most_abundant_isotope()),
            Self::Rb => Isotope::Rb(RubidiumIsotope::most_abundant_isotope()),
            Self::Sr => Isotope::Sr(StrontiumIsotope::most_abundant_isotope()),
            Self::Y => Isotope::Y(YttriumIsotope::most_abundant_isotope()),
            Self::Zr => Isotope::Zr(ZirconiumIsotope::most_abundant_isotope()),
            Self::Nb => Isotope::Nb(NiobiumIsotope::most_abundant_isotope()),
            Self::Mo => Isotope::Mo(MolybdenumIsotope::most_abundant_isotope()),
            Self::Tc => Isotope::Tc(TechnetiumIsotope::most_abundant_isotope()),
            Self::Ru => Isotope::Ru(RutheniumIsotope::most_abundant_isotope()),
            Self::Rh => Isotope::Rh(RhodiumIsotope::most_abundant_isotope()),
            Self::Pd => Isotope::Pd(PalladiumIsotope::most_abundant_isotope()),
            Self::Ag => Isotope::Ag(SilverIsotope::most_abundant_isotope()),
            Self::Cd => Isotope::Cd(CadmiumIsotope::most_abundant_isotope()),
            Self::In => Isotope::In(IndiumIsotope::most_abundant_isotope()),
            Self::Sn => Isotope::Sn(TinIsotope::most_abundant_isotope()),
            Self::Sb => Isotope::Sb(AntimonyIsotope::most_abundant_isotope()),
            Self::Te => Isotope::Te(TelluriumIsotope::most_abundant_isotope()),
            Self::I => Isotope::I(IodineIsotope::most_abundant_isotope()),
            Self::Xe => Isotope::Xe(XenonIsotope::most_abundant_isotope()),
            Self::Cs => Isotope::Cs(CaesiumIsotope::most_abundant_isotope()),
            Self::Ba => Isotope::Ba(BariumIsotope::most_abundant_isotope()),
            Self::La => Isotope::La(LanthanumIsotope::most_abundant_isotope()),
            Self::Ce => Isotope::Ce(CeriumIsotope::most_abundant_isotope()),
            Self::Pr => Isotope::Pr(PraseodymiumIsotope::most_abundant_isotope()),
            Self::Nd => Isotope::Nd(NeodymiumIsotope::most_abundant_isotope()),
            Self::Pm => Isotope::Pm(PromethiumIsotope::most_abundant_isotope()),
            Self::Sm => Isotope::Sm(SamariumIsotope::most_abundant_isotope()),
            Self::Eu => Isotope::Eu(EuropiumIsotope::most_abundant_isotope()),
            Self::Gd => Isotope::Gd(GadoliniumIsotope::most_abundant_isotope()),
            Self::Tb => Isotope::Tb(TerbiumIsotope::most_abundant_isotope()),
            Self::Dy => Isotope::Dy(DysprosiumIsotope::most_abundant_isotope()),
            Self::Ho => Isotope::Ho(HolmiumIsotope::most_abundant_isotope()),
            Self::Er => Isotope::Er(ErbiumIsotope::most_abundant_isotope()),
            Self::Tm => Isotope::Tm(ThuliumIsotope::most_abundant_isotope()),
            Self::Yb => Isotope::Yb(YtterbiumIsotope::most_abundant_isotope()),
            Self::Lu => Isotope::Lu(LutetiumIsotope::most_abundant_isotope()),
            Self::Hf => Isotope::Hf(HafniumIsotope::most_abundant_isotope()),
            Self::Ta => Isotope::Ta(TantalumIsotope::most_abundant_isotope()),
            Self::W => Isotope::W(TungstenIsotope::most_abundant_isotope()),
            Self::Re => Isotope::Re(RheniumIsotope::most_abundant_isotope()),
            Self::Os => Isotope::Os(OsmiumIsotope::most_abundant_isotope()),
            Self::Ir => Isotope::Ir(IridiumIsotope::most_abundant_isotope()),
            Self::Pt => Isotope::Pt(PlatinumIsotope::most_abundant_isotope()),
            Self::Au => Isotope::Au(GoldIsotope::most_abundant_isotope()),
            Self::Hg => Isotope::Hg(MercuryIsotope::most_abundant_isotope()),
            Self::Tl => Isotope::Tl(ThalliumIsotope::most_abundant_isotope()),
            Self::Pb => Isotope::Pb(LeadIsotope::most_abundant_isotope()),
            Self::Bi => Isotope::Bi(BismuthIsotope::most_abundant_isotope()),
            Self::Po => Isotope::Po(PoloniumIsotope::most_abundant_isotope()),
            Self::At => Isotope::At(AstatineIsotope::most_abundant_isotope()),
            Self::Rn => Isotope::Rn(RadonIsotope::most_abundant_isotope()),
            Self::Fr => Isotope::Fr(FranciumIsotope::most_abundant_isotope()),
            Self::Ra => Isotope::Ra(RadiumIsotope::most_abundant_isotope()),
            Self::Ac => Isotope::Ac(ActiniumIsotope::most_abundant_isotope()),
            Self::Th => Isotope::Th(ThoriumIsotope::most_abundant_isotope()),
            Self::Pa => Isotope::Pa(ProtactiniumIsotope::most_abundant_isotope()),
            Self::U => Isotope::U(UraniumIsotope::most_abundant_isotope()),
            Self::Np => Isotope::Np(NeptuniumIsotope::most_abundant_isotope()),
            Self::Pu => Isotope::Pu(PlutoniumIsotope::most_abundant_isotope()),
            Self::Am => Isotope::Am(AmericiumIsotope::most_abundant_isotope()),
            Self::Cm => Isotope::Cm(CuriumIsotope::most_abundant_isotope()),
            Self::Bk => Isotope::Bk(BerkeliumIsotope::most_abundant_isotope()),
            Self::Cf => Isotope::Cf(CaliforniumIsotope::most_abundant_isotope()),
            Self::Es => Isotope::Es(EinsteiniumIsotope::most_abundant_isotope()),
            Self::Fm => Isotope::Fm(FermiumIsotope::most_abundant_isotope()),
            Self::Md => Isotope::Md(MendeleviumIsotope::most_abundant_isotope()),
            Self::No => Isotope::No(NobeliumIsotope::most_abundant_isotope()),
            Self::Lr => Isotope::Lr(LawrenciumIsotope::most_abundant_isotope()),
            Self::Rf => Isotope::Rf(RutherfordiumIsotope::most_abundant_isotope()),
            Self::Db => Isotope::Db(DubniumIsotope::most_abundant_isotope()),
            Self::Sg => Isotope::Sg(SeaborgiumIsotope::most_abundant_isotope()),
            Self::Bh => Isotope::Bh(BohriumIsotope::most_abundant_isotope()),
            Self::Hs => Isotope::Hs(HassiumIsotope::most_abundant_isotope()),
            Self::Mt => Isotope::Mt(MeitneriumIsotope::most_abundant_isotope()),
            Self::Ds => Isotope::Ds(DarmstadtiumIsotope::most_abundant_isotope()),
            Self::Rg => Isotope::Rg(RoentgeniumIsotope::most_abundant_isotope()),
            Self::Cn => Isotope::Cn(CoperniciumIsotope::most_abundant_isotope()),
            Self::Nh => Isotope::Nh(NihoniumIsotope::most_abundant_isotope()),
            Self::Fl => Isotope::Fl(FleroviumIsotope::most_abundant_isotope()),
            Self::Mc => Isotope::Mc(MoscoviumIsotope::most_abundant_isotope()),
            Self::Lv => Isotope::Lv(LivermoriumIsotope::most_abundant_isotope()),
            Self::Ts => Isotope::Ts(TennessineIsotope::most_abundant_isotope()),
            Self::Og => Isotope::Og(OganessonIsotope::most_abundant_isotope()),
        }
    }
}

impl IsotopicComposition for crate::Element {
    fn isotopic_composition(&self) -> Option<f64> {
        self.most_abundant_isotope().isotopic_composition()
    }
}

impl RelativeAtomicMass for crate::Element {
    fn relative_atomic_mass(&self) -> f64 {
        self.most_abundant_isotope().relative_atomic_mass()
    }
}
