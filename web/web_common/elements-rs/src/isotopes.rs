//! Isotopes for all chemical elements with mass numbers and atomic masses.

mod display;
mod element_isotopes;
mod element_variant;
mod isotopic_composition;
mod mass_number;
mod most_abundant_isotope;
mod relative_atomic_mass;
mod try_from;
mod try_from_element;

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

/// Relative atomic mass (in atomic mass units).
pub trait RelativeAtomicMass {
    /// Returns the relative atomic mass in daltons (Da).
    fn relative_atomic_mass(&self) -> f64;
}

/// Mass number (number of protons + neutrons).
pub trait MassNumber {
    /// Returns the mass number (A).
    fn mass_number(&self) -> u16;
}

/// Element that an isotope belongs to.
pub trait ElementVariant {
    /// Returns the parent element.
    fn element(&self) -> crate::Element;
}

/// Natural abundance (isotopic composition) as a fraction.
pub trait IsotopicComposition {
    /// Returns `None` for isotopes without stable natural occurrence.
    fn isotopic_composition(&self) -> Option<f64>;
}

/// Most abundant naturally occurring isotope.
pub trait MostAbundantIsotope {
    /// Returns the most abundant isotope.
    fn most_abundant_isotope() -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// All known isotopes for all elements.
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
