//! Submodule providing the enumeration of `Isotopes` for each element.

mod display;
mod element_variant;
mod isotopic_composition;
mod mass_number;
mod relative_atomic_mass;
mod try_from_element;
use strum::IntoEnumIterator;

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
/// Enumeration of all isotopes of the elements in the periodic table.
pub enum Isotope {
    /// Isotopes of `HydrogenIsotope`
    H(HydrogenIsotope),
    /// Isotopes of Helium
    He(HeliumIsotope),
    /// Isotopes of Lithium
    Li(LithiumIsotope),
    /// Isotopes of Beryllium
    Be(BerylliumIsotope),
    /// Isotopes of Boron
    B(BoronIsotope),
    /// Isotopes of Carbon
    C(CarbonIsotope),
    /// Isotopes of Nitrogen
    N(NitrogenIsotope),
    /// Isotopes of Oxygen
    O(OxygenIsotope),
    /// Isotopes of Fluorine
    F(FluorineIsotope),
    /// Isotopes of Neon
    Ne(NeonIsotope),
    /// Isotopes of Sodium
    Na(SodiumIsotope),
    /// Isotopes of Magnesium
    Mg(MagnesiumIsotope),
    /// Isotopes of Aluminium
    Al(AluminiumIsotope),
    /// Isotopes of Silicon
    Si(SiliconIsotope),
    /// Isotopes of Phosphorus
    P(PhosphorusIsotope),
    /// Isotopes of Sulfur
    S(SulfurIsotope),
    /// Isotopes of Chlorine
    Cl(ChlorineIsotope),
    /// Isotopes of Argon
    Ar(ArgonIsotope),
    /// Isotopes of Potassium
    K(PotassiumIsotope),
    /// Isotopes of Calcium
    Ca(CalciumIsotope),
    /// Isotopes of Scandium
    Sc(ScandiumIsotope),
    /// Isotopes of Titanium
    Ti(TitaniumIsotope),
    /// Isotopes of Vanadium
    V(VanadiumIsotope),
    /// Isotopes of Chromium
    Cr(ChromiumIsotope),
    /// Isotopes of Manganese
    Mn(ManganeseIsotope),
    /// Isotopes of Iron
    Fe(IronIsotope),
    /// Isotopes of Cobalt
    Co(CobaltIsotope),
    /// Isotopes of Nickel
    Ni(NickelIsotope),
    /// Isotopes of Copper
    Cu(CopperIsotope),
    /// Isotopes of Zinc
    Zn(ZincIsotope),
    /// Isotopes of Gallium
    Ga(GalliumIsotope),
    /// Isotopes of Germanium
    Ge(GermaniumIsotope),
    /// Isotopes of Arsenic
    As(ArsenicIsotope),
    /// Isotopes of Selenium
    Se(SeleniumIsotope),
    /// Isotopes of Bromine
    Br(BromineIsotope),
    /// Isotopes of Krypton
    Kr(KryptonIsotope),
    /// Isotopes of Rubidium
    Rb(RubidiumIsotope),
    /// Isotopes of Strontium
    Sr(StrontiumIsotope),
    /// Isotopes of Yttrium
    Y(YttriumIsotope),
    /// Isotopes of Zirconium
    Zr(ZirconiumIsotope),
    /// Isotopes of Niobium
    Nb(NiobiumIsotope),
    /// Isotopes of Molybdenum
    Mo(MolybdenumIsotope),
    /// Isotopes of Technetium
    Tc(TechnetiumIsotope),
    /// Isotopes of Ruthenium
    Ru(RutheniumIsotope),
    /// Isotopes of Rhodium
    Rh(RhodiumIsotope),
    /// Isotopes of Palladium
    Pd(PalladiumIsotope),
    /// Isotopes of Silver
    Ag(SilverIsotope),
    /// Isotopes of Cadmium
    Cd(CadmiumIsotope),
    /// Isotopes of Indium
    In(IndiumIsotope),
    /// Isotopes of Tin
    Sn(TinIsotope),
    /// Isotopes of Antimony
    Sb(AntimonyIsotope),
    /// Isotopes of Tellurium
    Te(TelluriumIsotope),
    /// Isotopes of Iodine
    I(IodineIsotope),
    /// Isotopes of Xenon
    Xe(XenonIsotope),
    /// Isotopes of Caesium
    Cs(CaesiumIsotope),
    /// Isotopes of Barium
    Ba(BariumIsotope),
    /// Isotopes of Lanthanum
    La(LanthanumIsotope),
    /// Isotopes of Cerium
    Ce(CeriumIsotope),
    /// Isotopes of Praseodymium
    Pr(PraseodymiumIsotope),
    /// Isotopes of Neodymium
    Nd(NeodymiumIsotope),
    /// Isotopes of Promethium
    Pm(PromethiumIsotope),
    /// Isotopes of Samarium
    Sm(SamariumIsotope),
    /// Isotopes of Europium
    Eu(EuropiumIsotope),
    /// Isotopes of Gadolinium
    Gd(GadoliniumIsotope),
    /// Isotopes of Terbium
    Tb(TerbiumIsotope),
    /// Isotopes of Dysprosium
    Dy(DysprosiumIsotope),
    /// Isotopes of Holmium
    Ho(HolmiumIsotope),
    /// Isotopes of Erbium
    Er(ErbiumIsotope),
    /// Isotopes of Thulium
    Tm(ThuliumIsotope),
    /// Isotopes of Ytterbium
    Yb(YtterbiumIsotope),
    /// Isotopes of Lutetium
    Lu(LutetiumIsotope),
    /// Isotopes of Hafnium
    Hf(HafniumIsotope),
    /// Isotopes of Tantalum
    Ta(TantalumIsotope),
    /// Isotopes of Tungsten
    W(TungstenIsotope),
    /// Isotopes of Rhenium
    Re(RheniumIsotope),
    /// Isotopes of Osmium
    Os(OsmiumIsotope),
    /// Isotopes of Iridium
    Ir(IridiumIsotope),
    /// Isotopes of Platinum
    Pt(PlatinumIsotope),
    /// Isotopes of Gold
    Au(GoldIsotope),
    /// Isotopes of Mercury
    Hg(MercuryIsotope),
    /// Isotopes of Thallium
    Tl(ThalliumIsotope),
    /// Isotopes of Lead
    Pb(LeadIsotope),
    /// Isotopes of Bismuth
    Bi(BismuthIsotope),
    /// Isotopes of Polonium
    Po(PoloniumIsotope),
    /// Isotopes of Astatine
    At(AstatineIsotope),
    /// Isotopes of Radon
    Rn(RadonIsotope),
    /// Isotopes of Francium
    Fr(FranciumIsotope),
    /// Isotopes of Radium
    Ra(RadiumIsotope),
    /// Isotopes of Actinium
    Ac(ActiniumIsotope),
    /// Isotopes of Thorium
    Th(ThoriumIsotope),
    /// Isotopes of Protactinium
    Pa(ProtactiniumIsotope),
    /// Isotopes of Uranium
    U(UraniumIsotope),
    /// Isotopes of Neptunium
    Np(NeptuniumIsotope),
    /// Isotopes of Plutonium
    Pu(PlutoniumIsotope),
    /// Isotopes of Americium
    Am(AmericiumIsotope),
    /// Isotopes of Curium
    Cm(CuriumIsotope),
    /// Isotopes of Berkelium
    Bk(BerkeliumIsotope),
    /// Isotopes of Californium
    Cf(CaliforniumIsotope),
    /// Isotopes of Einsteinium
    Es(EinsteiniumIsotope),
    /// Isotopes of Fermium
    Fm(FermiumIsotope),
    /// Isotopes of Mendelevium
    Md(MendeleviumIsotope),
    /// Isotopes of Nobelium
    No(NobeliumIsotope),
    /// Isotopes of Lawrencium
    Lr(LawrenciumIsotope),
    /// Isotopes of Rutherfordium
    Rf(RutherfordiumIsotope),
    /// Isotopes of Dubnium
    Db(DubniumIsotope),
    /// Isotopes of Seaborgium
    Sg(SeaborgiumIsotope),
    /// Isotopes of Bohrium
    Bh(BohriumIsotope),
    /// Isotopes of Hassium
    Hs(HassiumIsotope),
    /// Isotopes of Meitnerium
    Mt(MeitneriumIsotope),
    /// Isotopes of Darmstadtium
    Ds(DarmstadtiumIsotope),
    /// Isotopes of Roentgenium
    Rg(RoentgeniumIsotope),
    /// Isotopes of Copernicium
    Cn(CoperniciumIsotope),
    /// Isotopes of Nihonium
    Nh(NihoniumIsotope),
    /// Isotopes of Flerovium
    Fl(FleroviumIsotope),
    /// Isotopes of Moscovium
    Mc(MoscoviumIsotope),
    /// Isotopes of Livermorium
    Lv(LivermoriumIsotope),
    /// Isotopes of Tennessine
    Ts(TennessineIsotope),
    /// Isotopes of Oganesson
    Og(OganessonIsotope),
}

impl crate::Element {
    /// Returns the most common isotope of the element.
    #[must_use]
    #[allow(clippy::too_many_lines)]
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

impl crate::Element {
    #[allow(clippy::too_many_lines)]
    /// Returns the isotopes of the element.
    pub fn isotopes(&self) -> Vec<Isotope> {
        match self {
            Self::H => HydrogenIsotope::iter().map(Isotope::H).collect(),
            Self::He => HeliumIsotope::iter().map(Isotope::He).collect(),
            Self::Li => LithiumIsotope::iter().map(Isotope::Li).collect(),
            Self::Be => BerylliumIsotope::iter().map(Isotope::Be).collect(),
            Self::B => BoronIsotope::iter().map(Isotope::B).collect(),
            Self::C => CarbonIsotope::iter().map(Isotope::C).collect(),
            Self::N => NitrogenIsotope::iter().map(Isotope::N).collect(),
            Self::O => OxygenIsotope::iter().map(Isotope::O).collect(),
            Self::F => FluorineIsotope::iter().map(Isotope::F).collect(),
            Self::Ne => NeonIsotope::iter().map(Isotope::Ne).collect(),
            Self::Na => SodiumIsotope::iter().map(Isotope::Na).collect(),
            Self::Mg => MagnesiumIsotope::iter().map(Isotope::Mg).collect(),
            Self::Al => AluminiumIsotope::iter().map(Isotope::Al).collect(),
            Self::Si => SiliconIsotope::iter().map(Isotope::Si).collect(),
            Self::P => PhosphorusIsotope::iter().map(Isotope::P).collect(),
            Self::S => SulfurIsotope::iter().map(Isotope::S).collect(),
            Self::Cl => ChlorineIsotope::iter().map(Isotope::Cl).collect(),
            Self::Ar => ArgonIsotope::iter().map(Isotope::Ar).collect(),
            Self::K => PotassiumIsotope::iter().map(Isotope::K).collect(),
            Self::Ca => CalciumIsotope::iter().map(Isotope::Ca).collect(),
            Self::Sc => ScandiumIsotope::iter().map(Isotope::Sc).collect(),
            Self::Ti => TitaniumIsotope::iter().map(Isotope::Ti).collect(),
            Self::V => VanadiumIsotope::iter().map(Isotope::V).collect(),
            Self::Cr => ChromiumIsotope::iter().map(Isotope::Cr).collect(),
            Self::Mn => ManganeseIsotope::iter().map(Isotope::Mn).collect(),
            Self::Fe => IronIsotope::iter().map(Isotope::Fe).collect(),
            Self::Co => CobaltIsotope::iter().map(Isotope::Co).collect(),
            Self::Ni => NickelIsotope::iter().map(Isotope::Ni).collect(),
            Self::Cu => CopperIsotope::iter().map(Isotope::Cu).collect(),
            Self::Zn => ZincIsotope::iter().map(Isotope::Zn).collect(),
            Self::Ga => GalliumIsotope::iter().map(Isotope::Ga).collect(),
            Self::Ge => GermaniumIsotope::iter().map(Isotope::Ge).collect(),
            Self::As => ArsenicIsotope::iter().map(Isotope::As).collect(),
            Self::Se => SeleniumIsotope::iter().map(Isotope::Se).collect(),
            Self::Br => BromineIsotope::iter().map(Isotope::Br).collect(),
            Self::Kr => KryptonIsotope::iter().map(Isotope::Kr).collect(),
            Self::Rb => RubidiumIsotope::iter().map(Isotope::Rb).collect(),
            Self::Sr => StrontiumIsotope::iter().map(Isotope::Sr).collect(),
            Self::Y => YttriumIsotope::iter().map(Isotope::Y).collect(),
            Self::Zr => ZirconiumIsotope::iter().map(Isotope::Zr).collect(),
            Self::Nb => NiobiumIsotope::iter().map(Isotope::Nb).collect(),
            Self::Mo => MolybdenumIsotope::iter().map(Isotope::Mo).collect(),
            Self::Tc => TechnetiumIsotope::iter().map(Isotope::Tc).collect(),
            Self::Ru => RutheniumIsotope::iter().map(Isotope::Ru).collect(),
            Self::Rh => RhodiumIsotope::iter().map(Isotope::Rh).collect(),
            Self::Pd => PalladiumIsotope::iter().map(Isotope::Pd).collect(),
            Self::Ag => SilverIsotope::iter().map(Isotope::Ag).collect(),
            Self::Cd => CadmiumIsotope::iter().map(Isotope::Cd).collect(),
            Self::In => IndiumIsotope::iter().map(Isotope::In).collect(),
            Self::Sn => TinIsotope::iter().map(Isotope::Sn).collect(),
            Self::Sb => AntimonyIsotope::iter().map(Isotope::Sb).collect(),
            Self::Te => TelluriumIsotope::iter().map(Isotope::Te).collect(),
            Self::I => IodineIsotope::iter().map(Isotope::I).collect(),
            Self::Xe => XenonIsotope::iter().map(Isotope::Xe).collect(),
            Self::Cs => CaesiumIsotope::iter().map(Isotope::Cs).collect(),
            Self::Ba => BariumIsotope::iter().map(Isotope::Ba).collect(),
            Self::La => LanthanumIsotope::iter().map(Isotope::La).collect(),
            Self::Ce => CeriumIsotope::iter().map(Isotope::Ce).collect(),
            Self::Pr => PraseodymiumIsotope::iter().map(Isotope::Pr).collect(),
            Self::Nd => NeodymiumIsotope::iter().map(Isotope::Nd).collect(),
            Self::Pm => PromethiumIsotope::iter().map(Isotope::Pm).collect(),
            Self::Sm => SamariumIsotope::iter().map(Isotope::Sm).collect(),
            Self::Eu => EuropiumIsotope::iter().map(Isotope::Eu).collect(),
            Self::Gd => GadoliniumIsotope::iter().map(Isotope::Gd).collect(),
            Self::Tb => TerbiumIsotope::iter().map(Isotope::Tb).collect(),
            Self::Dy => DysprosiumIsotope::iter().map(Isotope::Dy).collect(),
            Self::Ho => HolmiumIsotope::iter().map(Isotope::Ho).collect(),
            Self::Er => ErbiumIsotope::iter().map(Isotope::Er).collect(),
            Self::Tm => ThuliumIsotope::iter().map(Isotope::Tm).collect(),
            Self::Yb => YtterbiumIsotope::iter().map(Isotope::Yb).collect(),
            Self::Lu => LutetiumIsotope::iter().map(Isotope::Lu).collect(),
            Self::Hf => HafniumIsotope::iter().map(Isotope::Hf).collect(),
            Self::Ta => TantalumIsotope::iter().map(Isotope::Ta).collect(),
            Self::W => TungstenIsotope::iter().map(Isotope::W).collect(),
            Self::Re => RheniumIsotope::iter().map(Isotope::Re).collect(),
            Self::Os => OsmiumIsotope::iter().map(Isotope::Os).collect(),
            Self::Ir => IridiumIsotope::iter().map(Isotope::Ir).collect(),
            Self::Pt => PlatinumIsotope::iter().map(Isotope::Pt).collect(),
            Self::Au => GoldIsotope::iter().map(Isotope::Au).collect(),
            Self::Hg => MercuryIsotope::iter().map(Isotope::Hg).collect(),
            Self::Tl => ThalliumIsotope::iter().map(Isotope::Tl).collect(),
            Self::Pb => LeadIsotope::iter().map(Isotope::Pb).collect(),
            Self::Bi => BismuthIsotope::iter().map(Isotope::Bi).collect(),
            Self::Po => PoloniumIsotope::iter().map(Isotope::Po).collect(),
            Self::At => AstatineIsotope::iter().map(Isotope::At).collect(),
            Self::Rn => RadonIsotope::iter().map(Isotope::Rn).collect(),
            Self::Fr => FranciumIsotope::iter().map(Isotope::Fr).collect(),
            Self::Ra => RadiumIsotope::iter().map(Isotope::Ra).collect(),
            Self::Ac => ActiniumIsotope::iter().map(Isotope::Ac).collect(),
            Self::Th => ThoriumIsotope::iter().map(Isotope::Th).collect(),
            Self::Pa => ProtactiniumIsotope::iter().map(Isotope::Pa).collect(),
            Self::U => UraniumIsotope::iter().map(Isotope::U).collect(),
            Self::Np => NeptuniumIsotope::iter().map(Isotope::Np).collect(),
            Self::Pu => PlutoniumIsotope::iter().map(Isotope::Pu).collect(),
            Self::Am => AmericiumIsotope::iter().map(Isotope::Am).collect(),
            Self::Cm => CuriumIsotope::iter().map(Isotope::Cm).collect(),
            Self::Bk => BerkeliumIsotope::iter().map(Isotope::Bk).collect(),
            Self::Cf => CaliforniumIsotope::iter().map(Isotope::Cf).collect(),
            Self::Es => EinsteiniumIsotope::iter().map(Isotope::Es).collect(),
            Self::Fm => FermiumIsotope::iter().map(Isotope::Fm).collect(),
            Self::Md => MendeleviumIsotope::iter().map(Isotope::Md).collect(),
            Self::No => NobeliumIsotope::iter().map(Isotope::No).collect(),
            Self::Lr => LawrenciumIsotope::iter().map(Isotope::Lr).collect(),
            Self::Rf => RutherfordiumIsotope::iter().map(Isotope::Rf).collect(),
            Self::Db => DubniumIsotope::iter().map(Isotope::Db).collect(),
            Self::Sg => SeaborgiumIsotope::iter().map(Isotope::Sg).collect(),
            Self::Bh => BohriumIsotope::iter().map(Isotope::Bh).collect(),
            Self::Hs => HassiumIsotope::iter().map(Isotope::Hs).collect(),
            Self::Mt => MeitneriumIsotope::iter().map(Isotope::Mt).collect(),
            Self::Ds => DarmstadtiumIsotope::iter().map(Isotope::Ds).collect(),
            Self::Rg => RoentgeniumIsotope::iter().map(Isotope::Rg).collect(),
            Self::Cn => CoperniciumIsotope::iter().map(Isotope::Cn).collect(),
            Self::Nh => NihoniumIsotope::iter().map(Isotope::Nh).collect(),
            Self::Fl => FleroviumIsotope::iter().map(Isotope::Fl).collect(),
            Self::Mc => MoscoviumIsotope::iter().map(Isotope::Mc).collect(),
            Self::Lv => LivermoriumIsotope::iter().map(Isotope::Lv).collect(),
            Self::Ts => TennessineIsotope::iter().map(Isotope::Ts).collect(),
            Self::Og => OganessonIsotope::iter().map(Isotope::Og).collect(),
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
