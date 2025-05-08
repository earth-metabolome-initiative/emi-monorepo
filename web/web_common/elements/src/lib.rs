#![doc = include_str!("../README.md")]

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

#[cfg(feature = "pgrx")]
use pgrx::FromDatum;

mod as_ref;
pub mod diesel_impls;
mod display;
pub mod errors;
mod from_str;
mod into;
mod isotopes;
mod name;
mod oxidation_states;
mod standard_atomic_weight;
mod try_from;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, strum_macros::EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
#[cfg_attr(feature = "diesel", derive(diesel::FromSqlRow, diesel::AsExpression))]
#[cfg_attr(
	feature = "diesel",
	diesel(sql_type = crate::diesel_impls::Element)
)]
/// Enum representing the elements of the periodic table.
pub enum Element {
    /// Hydrogen
    H,
    /// Helium
    He,
    /// Lithium
    Li,
    /// Beryllium
    Be,
    /// Boron
    B,
    /// Carbon
    C,
    /// Nitrogen
    N,
    /// Oxygen
    O,
    /// Fluorine
    F,
    /// Neon
    Ne,
    /// Sodium
    Na,
    /// Magnesium
    Mg,
    /// Aluminium
    Al,
    /// Silicon
    Si,
    /// Phosphorus
    P,
    /// Sulfur
    S,
    /// Chlorine
    Cl,
    /// Argon
    Ar,
    /// Potassium
    K,
    /// Calcium
    Ca,
    /// Scandium
    Sc,
    /// Titanium
    Ti,
    /// Vanadium
    V,
    /// Chromium
    Cr,
    /// Manganese
    Mn,
    /// Iron
    Fe,
    /// Cobalt
    Co,
    /// Nickel
    Ni,
    /// Copper
    Cu,
    /// Zinc
    Zn,
    /// Gallium
    Ga,
    /// Germanium
    Ge,
    /// Arsenic
    As,
    /// Selenium
    Se,
    /// Bromine
    Br,
    /// Krypton
    Kr,
    /// Rubidium
    Rb,
    /// Strontium
    Sr,
    /// Yttrium
    Y,
    /// Zirconium
    Zr,
    /// Niobium
    Nb,
    /// Molybdenum
    Mo,
    /// Technetium
    Tc,
    /// Ruthenium
    Ru,
    /// Rhodium
    Rh,
    /// Palladium
    Pd,
    /// Silver
    Ag,
    /// Cadmium
    Cd,
    /// Indium
    In,
    /// Tin
    Sn,
    /// Antimony
    Sb,
    /// Tellurium
    Te,
    /// Iodine
    I,
    /// Xenon
    Xe,
    /// Caesium
    Cs,
    /// Barium
    Ba,
    /// Lanthanum
    La,
    /// Cerium
    Ce,
    /// Praseodymium
    Pr,
    /// Neodymium
    Nd,
    /// Promethium
    Pm,
    /// Samarium
    Sm,
    /// Europium
    Eu,
    /// Gadolinium
    Gd,
    /// Terbium
    Tb,
    /// Dysprosium
    Dy,
    /// Holmium
    Ho,
    /// Erbium
    Er,
    /// Thulium
    Tm,
    /// Ytterbium
    Yb,
    /// Lutetium
    Lu,
    /// Hafnium
    Hf,
    /// Tantalum
    Ta,
    /// Tungsten
    W,
    /// Rhenium
    Re,
    /// Osmium
    Os,
    /// Iridium
    Ir,
    /// Platinum
    Pt,
    /// Gold
    Au,
    /// Mercury
    Hg,
    /// Thallium
    Tl,
    /// Lead
    Pb,
    /// Bismuth
    Bi,
    /// Polonium
    Po,
    /// Astatine
    At,
    /// Radon
    Rn,
    /// Francium
    Fr,
    /// Radium
    Ra,
    /// Actinium
    Ac,
    /// Thorium
    Th,
    /// Protactinium
    Pa,
    /// Uranium
    U,
    /// Neptunium
    Np,
    /// Plutonium
    Pu,
    /// Americium
    Am,
    /// Curium
    Cm,
    /// Berkelium
    Bk,
    /// Californium
    Cf,
    /// Einsteinium
    Es,
    /// Fermium
    Fm,
    /// Mendelevium
    Md,
    /// Nobelium
    No,
    /// Lawrencium
    Lr,
    /// Rutherfordium
    Rf,
    /// Dubnium
    Db,
    /// Seaborgium
    Sg,
    /// Bohrium
    Bh,
    /// Hassium
    Hs,
    /// Meitnerium
    Mt,
    /// Darmstadtium
    Ds,
    /// Roentgenium
    Rg,
    /// Copernicium
    Cn,
    /// Nihonium
    Nh,
    /// Flerovium
    Fl,
    /// Moscovium
    Mc,
    /// Livermorium
    Lv,
    /// Tennessine
    Ts,
    /// Oganesson
    Og,
}
