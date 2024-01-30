use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Adducts {
    #[default]
    H,
    K,
    Na,
    Nh4,
    Cl,
    Water,
}
